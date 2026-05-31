use edisondb::{executor::EqlExecutor, eql::parse};
use rpassword::read_password;
use rustyline::{DefaultEditor, error::ReadlineError};
use std::io::{self, Write};

const DB_PATH: &str = "edison.redb";
const HISTORY: &str = ".eql_history";

fn main() {
    print_banner();

    // -- Session credentials -----------------------------------------------
    let owner_id = prompt_line("Owner ID : ");
    if owner_id.is_empty() {
        eprintln!("Owner ID cannot be empty.");
        return;
    }
    print!("Password : ");
    io::stdout().flush().ok();
    let password = read_password().unwrap_or_default();
    if password.is_empty() {
        eprintln!("Password cannot be empty.");
        return;
    }

    // -- Open database ------------------------------------------------------
    let mut ex = match EqlExecutor::open(DB_PATH, &owner_id, &password) {
        Ok(e)  => e,
        Err(e) => { eprintln!("Failed to open database: {e}"); return; }
    };

    println!("\nDatabase : {DB_PATH}");
    println!("Owner    : {owner_id}");
    println!("Type EQL statements or 'help'. Ctrl-C / Ctrl-D to exit.\n");

    // -- REPL ---------------------------------------------------------------
    let mut rl = DefaultEditor::new().expect("readline init failed");
    let _ = rl.load_history(HISTORY);

    'repl: loop {
        match rl.readline("eql> ") {
            Ok(line) => {
                // Split on newlines to handle multi-line paste gracefully.
                // Each physical line is treated as one EQL statement.
                for raw in line.split('\n') {
                    let stmt = raw.trim().to_string();
                    if stmt.is_empty() { continue; }

                    let _ = rl.add_history_entry(&stmt);

                    if stmt.eq_ignore_ascii_case("help") {
                        print_help();
                        continue;
                    }
                    if stmt.eq_ignore_ascii_case("exit")
                        || stmt.eq_ignore_ascii_case("quit") {
                        break 'repl;
                    }

                    match parse(&stmt) {
                        Err(e)   => eprintln!("Parse error: {e}"),
                        Ok(stmt) => match ex.execute(stmt) {
                            Ok(result) => println!("{result}"),
                            Err(e)     => eprintln!("Error: {e}"),
                        },
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(e) => { eprintln!("Readline error: {e}"); break; }
        }
    }

    let _ = rl.save_history(HISTORY);
    println!("\nSession closed. Goodbye.");
}

// -- Helpers -----------------------------------------------------------------
fn prompt_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().ok();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn print_banner() {
    println!("╔══════════════════════════════════════╗");
    println!("║        EdisonDB  —  EQL Shell        ║");
    println!("║   Sovereign. Encrypted. Yours.       ║");
    println!("╚══════════════════════════════════════╝");
    println!();
}

fn print_help() {
    println!("  WRITE <id> TIER <CRITICAL|PERSONAL|NOISE> <payload>");
    println!("  READ  <id>");
    println!("  LIST  [TIER <CRITICAL|PERSONAL|NOISE>]");
    println!("  DELETE <id>");
    println!("  AUDIT  [<id>]");
    println!("  help | exit | quit");
}
