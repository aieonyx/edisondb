use edisondb::{Store, Record, DataTier};

fn main() {
    println!("EdisonDB v0.1.0");
    println!("---------------");

    let db_path = "edison.db.json";
    let mut store = Store::new();

    // Write a test record
    let record = Record::new(
        1,
        DataTier::Personal,
        "owner_ed",
        b"sovereign data test".to_vec(),
    ).unwrap();

    store.write(record);
    println!("Record written.");

    // Read it back
    match store.read(1, "owner_ed") {
        Ok(r) => println!("Read ok — owner: {}", r.owner_id),
        Err(e) => println!("Read failed: {:?}", e),
    }

    // Try unauthorized access
    match store.read(1, "attacker") {
        Ok(_) => println!("ERROR — attacker got access"),
        Err(_) => println!("Access denied for attacker — correct."),
    }

    // Save to disk
    store.save(db_path).unwrap();
    println!("Saved to {}", db_path);

    // Audit log count
    println!("Audit entries: {}", store.audit_count());
}
