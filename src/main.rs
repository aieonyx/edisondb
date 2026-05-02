use clap::{Parser, Subcommand};
use edisondb::{Store, Record, DataTier, encrypt_payload, decrypt_payload, derive_key};
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "edisondb")]
#[command(about = "EdisonDB - Sovereign data storage")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    Read {
        #[arg(long)]
        id: u64,
        #[arg(long)]
        requester: String,
    },
    Audit,
}

const DB_PATH: &str = "edison.db.json";
const SALT: &[u8; 32] = b"aieonyx_sovereign_salt_v1_fixed!";

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
            let key = derive_key(&password, SALT);
            let encrypted = encrypt_payload(data.as_bytes(), &key).unwrap();

            let mut store = load_store();
            match Record::new(id, data_tier, &owner, encrypted) {
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
                    let key = derive_key(&password, SALT);
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

        Commands::Audit => {
            let store = load_store();
            println!("Audit log entries: {}", store.audit_count());
        }
    }
}

fn load_store() -> Store {
    Store::load(DB_PATH).unwrap_or_else(|_| Store::new())
}
