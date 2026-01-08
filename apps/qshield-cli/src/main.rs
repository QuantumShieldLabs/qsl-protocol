use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod actor;
mod commands;
mod config;
mod fsutil;
mod relay_client;
mod store;
mod util;

#[derive(Parser)]
#[command(
    name = "qshield",
    version,
    about = "QuantumShield demo CLI (non-production)"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialize a local demo store and config
    Init {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
        /// Relay URL to store in config (default: http://127.0.0.1:18080)
        #[arg(long)]
        relay_url: Option<String>,
        /// Relay bearer token to store in config (overrides QSHIELD_RELAY_TOKEN)
        #[arg(long)]
        relay_token: Option<String>,
        /// Enable size-bucket padding in the demo transport (default: false)
        #[arg(long, default_value_t = false)]
        padding_enable: bool,
        /// Comma-separated padding bucket sizes in bytes (e.g., "256,512,1024")
        #[arg(long)]
        padding_buckets: Option<String>,
    },
    /// Relay subcommands (demo-local only)
    Relay {
        #[command(subcommand)]
        command: RelayCommand,
    },
    /// Show local store/config status
    Status {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
    },
    /// Rotate (wipe) the local demo store artifacts
    Rotate {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
    },
    /// Register a contact bundle with the relay
    Register {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
        /// Local demo identifier
        #[arg(long)]
        id: String,
    },
    /// Establish a Suite-2 session
    Establish {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
        /// Peer identifier
        #[arg(long)]
        peer: String,
        /// Role override (A or B). Default derives from id order.
        #[arg(long)]
        role: Option<String>,
        /// Override unauthenticated establishment (demo-only; prints warning)
        #[arg(long, default_value_t = false)]
        demo_unauthenticated_override: bool,
        /// Suppress first-establish identity verification warning (demo-only)
        #[arg(long, default_value_t = false)]
        demo_identity_verified: bool,
    },
    /// Send a message
    Send {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
        /// Peer identifier
        #[arg(long)]
        peer: String,
        /// Plaintext message
        #[arg(long)]
        text: String,
        /// Override unauthenticated establishment (demo-only; prints warning)
        #[arg(long, default_value_t = false)]
        demo_unauthenticated_override: bool,
    },
    /// Receive messages
    Recv {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
        /// Max messages to retrieve
        #[arg(long, default_value_t = 10)]
        max: u32,
        /// Override unauthenticated establishment (demo-only; prints warning)
        #[arg(long, default_value_t = false)]
        demo_unauthenticated_override: bool,
    },
}

#[derive(Subcommand)]
enum RelayCommand {
    /// Start the local relay (demo-only)
    Serve {
        /// Listen address (local only)
        #[arg(long, default_value = "127.0.0.1:18080")]
        listen: String,
        /// Allow non-loopback bind (demo-only; default OFF)
        #[arg(long, default_value_t = false)]
        allow_public: bool,
        /// Acknowledge unsafe public bind (required with --allow-public)
        #[arg(long, default_value_t = false)]
        i_understand_this_is_unsafe: bool,
    },
    /// Send a raw message blob to the relay queue (demo-only)
    Send {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
        /// Recipient id
        #[arg(long)]
        to: String,
        /// Sender id
        #[arg(long)]
        from: String,
        /// Message blob (hex or base64 string)
        #[arg(long)]
        msg: String,
    },
    /// Poll raw message blobs from the relay queue (demo-only)
    Poll {
        /// Path to local store directory
        #[arg(long)]
        store: PathBuf,
        /// Recipient id
        #[arg(long)]
        id: String,
        /// Max messages to retrieve
        #[arg(long, default_value_t = 1)]
        max: u32,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Init {
            store,
            relay_url,
            relay_token,
            padding_enable,
            padding_buckets,
        } => commands::init::run(
            &store,
            relay_url,
            relay_token,
            padding_enable,
            padding_buckets,
        ),
        Command::Relay { command } => match command {
            RelayCommand::Serve {
                listen,
                allow_public,
                i_understand_this_is_unsafe,
            } => commands::relay::serve(&listen, allow_public, i_understand_this_is_unsafe),
            RelayCommand::Send {
                store,
                to,
                from,
                msg,
            } => commands::relay::send(&store, &to, &from, &msg),
            RelayCommand::Poll { store, id, max } => commands::relay::poll(&store, &id, max),
        },
        Command::Status { store } => commands::status::run(&store),
        Command::Rotate { store } => commands::rotate::run(&store),
        Command::Register { store, id } => commands::register::run(&store, &id),
        Command::Establish {
            store,
            peer,
            role,
            demo_unauthenticated_override,
            demo_identity_verified,
        } => commands::establish::run(
            &store,
            &peer,
            role,
            demo_unauthenticated_override,
            demo_identity_verified,
        ),
        Command::Send {
            store,
            peer,
            text,
            demo_unauthenticated_override,
        } => commands::send::run(&store, &peer, &text, demo_unauthenticated_override),
        Command::Recv {
            store,
            max,
            demo_unauthenticated_override,
        } => commands::recv::run(&store, max, demo_unauthenticated_override),
    };

    if let Err(err) = result {
        eprintln!("error: {err}");
        std::process::exit(2);
    }
}
