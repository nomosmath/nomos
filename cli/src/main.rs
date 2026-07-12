use clap::Parser;

#[derive(Parser)]
#[command(name = "nomos", version, about = "Nomos proof verification node")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Start a validator node
    Run {
        #[arg(long, default_value = "config.toml")]
        config: String,
        #[arg(long)]
        rollup_mode: bool,
    },
    /// Submit a proof for verification
    Submit {
        #[arg(long)]
        proof_path: String,
        #[arg(long)]
        kind: String,
    },
    /// Query proof status
    Status {
        #[arg(long)]
        proof_id: String,
    },
}

#[tokio::main]
async fn main() {
    tracing_subscriber::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { config, rollup_mode } => {
            eprintln!("starting validator node (config={}, rollup={})", config, rollup_mode);
            eprintln!("not yet implemented — see docs/architecture.md");
        }
        Commands::Submit { proof_path, kind } => {
            eprintln!("submitting proof: {} (kind={})", proof_path, kind);
            eprintln!("not yet implemented");
        }
        Commands::Status { proof_id } => {
            eprintln!("querying proof: {}", proof_id);
            eprintln!("not yet implemented");
        }
    }
}

// --metrics flag

