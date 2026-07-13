fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("CARGO_FEATURE_SERVER").is_ok() {
        let protoc_path = protoc_bin_vendored::protoc_bin_path()
            .expect("protoc-bin-vendored: failed to locate bundled protoc binary");
        unsafe { std::env::set_var("PROTOC", protoc_path); }
        tonic_prost_build::compile_protos("proto/edisondb.proto")?;
        println!("cargo:rerun-if-changed=proto/edisondb.proto");
    }
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
