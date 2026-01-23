use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "qsc", version, about = "QSC client (Phase 1 scaffold)")]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print a deterministic status summary (no secrets, no timestamps).
    Status,
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        None => {
            // Shell-first UX expects help by default.
            // clap already prints help on -h/--help; here we emit a stable marker.
            println!("QSC_MARK/1 event=help_stub");
        }
        Some(Cmd::Status) => {
            // Deterministic, non-sensitive, CI-stable output.
            println!("QSC_MARK/1 event=status ok=true locked=unknown");
        }
    }
}
