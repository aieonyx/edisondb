use clap::{Parser, Subcommand};
use edisondb::{Store, Record, DataTier, encrypt_payload, decrypt_payload, derive_key};
use std::io::{self, Write};
use rand::RngCore;

#[derive(Parser)]
#[command(name = "edisondb")]
#[command(about = "EdisonDB - Sovereign data storage")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Write an encrypted record to the store
    Write {
        #[arg(long)]
        id: u64,
        #[arg(long)]
        owner: String,
        #[arg(long)]
        tier: String,
        #[arg(long)]
        data: String,
    },
    /// Read and decrypt a record (owner only)
    Read {
        #[arg(long)]
        id: u64,
        #[arg(long)]
        requester: String,
    },
    /// List all records belonging to an owner
    List {
        #[arg(long)]
        owner: String,
    },
    /// Delete a record (owner only)
    Delete {
        #[arg(long)]
        id: u64,
        #[arg(long)]
        owner: String,
    },
    /// Show the full audit log
    Audit,
}
const DB_PATH: &str = "edison.db.json";

fn prompt_password(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    password.trim().to_string()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Write { id, owner, tier, data } => {
            let data_tier = match tier.as_str() {
                "critical" => DataTier::Critical,
                "personal" => DataTier::Personal,
                "noise"    => DataTier::Noise,
                _ => {
                    println!("Unknown tier. Use: critical, personal, noise");
                    return;
                }
            };

            let password = prompt_password("Enter owner password: ");
            let mut salt = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut salt);
            let key = derive_key(&password, &salt);
            let encrypted = encrypt_payload(data.as_bytes(), &key).unwrap();

            let mut store = load_store();
            match Record::new(id, data_tier, &owner, encrypted, salt) {
                Ok(record) => {
                    store.write(record);
                    store.save(DB_PATH).unwrap();
                    println!("Record {} written and encrypted.", id);
                }
                Err(e) => println!("Error: {:?}", e),
            }
        }

        Commands::Read { id, requester } => {
            let mut store = load_store();
            match store.read(id, &requester) {
                Ok(record) => {
                    let password = prompt_password("Enter owner password: ");
                    let key = derive_key(&password, &record.salt);
                    match decrypt_payload(&record.payload, &key) {
                        Ok(decrypted) => {
                            let data = String::from_utf8_lossy(&decrypted);
                            println!("Record {}:", id);
                            println!("  Owner: {}", record.owner_id);
                            println!("  Tier:  {:?}", record.tier);
                            println!("  Data:  {}", data);
                        }
                        Err(_) => println!("Wrong password — cannot decrypt."),
                    }
                    store.save(DB_PATH).unwrap();
                }
                Err(e) => println!("Access denied: {:?}", e),
            }
        }

        Commands::List { owner } => {
            let store = load_store();
            let records = store.list_by_owner(&owner);
            if records.is_empty() {
                println!("No records found for owner: {}", owner);
            } else {
                println!("Records for {}:", owner);
                for r in records {
                    println!("  ID: {}  Tier: {:?}", r.id, r.tier);
                }
            }
        }

        Commands::Delete { id, owner } => {
            let mut store = load_store();
            match store.delete(id, &owner) {
                Ok(()) => {
                    store.save(DB_PATH).unwrap();
                    println!("Record {} deleted.", id);
                }
                Err(e) => println!("Delete failed: {:?}", e),
            }
        }

Commands::Audit => {
            let store = load_store();
            let entries = store.audit_entries();
            if entries.is_empty() {
                println!("No audit entries.");
            } else {
                println!("Audit log ({} entries):", entries.len());
                println!("  {:<8} {:<12} {:<15} {}",
                    "Record", "By", "Action", "Time");
                println!("  {}", "-".repeat(50));
                for e in entries {
                    let action = match e.action {
                        edisondb::AuditAction::Write => "WRITE",
                        edisondb::AuditAction::ReadGranted => "READ_OK",
                        edisondb::AuditAction::ReadDenied => "READ_DENIED",
                    };
                    println!("  {:<8} {:<12} {:<15} {}",
                        e.record_id, e.requester_id, action, e.timestamp);
                }
            }
        }
    }
}

fn load_store() -> Store {
    Store::load(DB_PATH).unwrap_or_else(|_| Store::new())
}
