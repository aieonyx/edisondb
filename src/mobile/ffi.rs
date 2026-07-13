// Copyright (c) 2026 Edison Lepiten / AIEONYX
// License: Apache-2.0
//
// EdisonDB Mobile — C-ABI FFI bridge
// Enabled only under the `mobile` feature flag.
// gRPC/tonic is excluded in mobile mode; this is the sole entry surface.

#![allow(clippy::missing_safety_doc)]

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

use crate::mobile::{DbError, MobileDb};

/// Opaque handle passed across the JNI boundary.
pub struct DbHandle {
    inner: Mutex<MobileDb>,
}

/// Open (or create) an EdisonDB embedded store at `path`.
/// Returns null on failure. Caller must call `edisondb_close` to free.
#[no_mangle]
pub unsafe extern "C" fn edisondb_open(path: *const c_char) -> *mut DbHandle {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let path_str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return std::ptr::null_mut(),
    };
    match MobileDb::open(&path_str) {
        Ok(db) => Box::into_raw(Box::new(DbHandle {
            inner: Mutex::new(db),
        })),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Insert `value` at `key`.
/// `arpi_header` must point to exactly 78 bytes (ARPi provenance header).
/// Returns 0 on success, negative error code on failure.
#[no_mangle]
pub unsafe extern "C" fn edisondb_insert(
    db: *mut DbHandle,
    key: *const c_char,
    value: *const c_char,
    arpi_header: *const u8,
) -> i32 {
    if db.is_null() || key.is_null() || value.is_null() || arpi_header.is_null() {
        return -1;
    }
    let key_str = match CStr::from_ptr(key).to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return -2,
    };
    let value_str = match CStr::from_ptr(value).to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return -2,
    };
    // SAFETY: caller guarantees arpi_header is 78 bytes.
    let header_bytes = std::slice::from_raw_parts(arpi_header, 78);
    let handle = &*db;
    match handle.inner.lock() {
        Ok(mut inner) => match inner.insert(&key_str, &value_str, header_bytes) {
            Ok(_) => 0,
            Err(DbError::KeyExists) => -10,
            Err(_) => -3,
        },
        Err(_) => -4,
    }
}

/// Query the value for `key`.
/// Returns a malloc'd C string that must be freed with `edisondb_free_string`.
/// Returns null if not found or on error.
#[no_mangle]
pub unsafe extern "C" fn edisondb_query(
    db: *mut DbHandle,
    key: *const c_char,
) -> *mut c_char {
    if db.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }
    let key_str = match CStr::from_ptr(key).to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return std::ptr::null_mut(),
    };
    let handle = &*db;
    match handle.inner.lock() {
        Ok(inner) => match inner.query(&key_str) {
            Ok(Some(val)) => match CString::new(val) {
                Ok(cs) => cs.into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            _ => std::ptr::null_mut(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

/// Delete the entry at `key`.
/// Returns 0 on success, -1 if not found, negative on error.
#[no_mangle]
pub unsafe extern "C" fn edisondb_delete(
    db: *mut DbHandle,
    key: *const c_char,
) -> i32 {
    if db.is_null() || key.is_null() {
        return -1;
    }
    let key_str = match CStr::from_ptr(key).to_str() {
        Ok(s) => s.to_owned(),
        Err(_) => return -2,
    };
    let handle = &*db;
    match handle.inner.lock() {
        Ok(mut inner) => match inner.delete(&key_str) {
            Ok(true) => 0,
            Ok(false) => -1,
            Err(_) => -3,
        },
        Err(_) => -4,
    }
}

/// Close the database and free the handle.
/// After this call `db` is invalid; do not use it.
#[no_mangle]
pub unsafe extern "C" fn edisondb_close(db: *mut DbHandle) {
    if !db.is_null() {
        drop(Box::from_raw(db));
    }
}

/// Free a string previously returned by `edisondb_query`.
#[no_mangle]
pub unsafe extern "C" fn edisondb_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}
