use clap::Parser;
use clap::Subcommand;
use clap_complete::Shell;

/// CLI for TxODDS/TxLINE Sports Data API
#[derive(Debug, Parser)]
#[command(name = "txodds", version, about)]
pub struct Cli {
    /// Path to config file
    #[arg(long, global = true)]
    pub config: Option<String>,

    /// Network (mainnet or devnet)
    #[arg(long, global = true)]
    pub network: Option<String>,

    /// Raw JSON output (no pretty-print)
    #[arg(long, global = true)]
    pub raw: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Authentication commands
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    /// Subscribe on-chain
    Subscribe {
        /// Path to Solana keypair file
        #[arg(long)]
        keypair: String,
        /// Service level ID
        #[arg(long)]
        service_level: u32,
        /// Number of weeks to subscribe
        #[arg(long)]
        weeks: u32,
        /// Solana RPC URL
        #[arg(long)]
        rpc: String,
    },
    /// Fixture commands
    Fixtures {
        #[command(subcommand)]
        command: FixturesCommand,
    },
    /// Odds commands
    Odds {
        #[command(subcommand)]
        command: OddsCommand,
    },
    /// Scores commands
    Scores {
        #[command(subcommand)]
        command: ScoresCommand,
    },
    /// Generate shell completions
    Completions {
        /// Shell type
        #[arg(long, value_enum)]
        shell: Shell,
    },
}

#[derive(Debug, Subcommand)]
pub enum AuthCommand {
    /// Request a guest JWT token
    Guest,
    /// Activate a guest token via on-chain transaction
    Activate {
        /// Transaction signature from guest token purchase
        #[arg(long)]
        tx_sig: String,
        /// Path to the Solana keypair file
        #[arg(long)]
        keypair: String,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum FixturesCommand {
    /// Get a snapshot of all fixtures
    Snapshot {
        /// Filter by competition ID
        #[arg(long)]
        competition_id: Option<u64>,
    },
    /// Get fixture updates for a specific time window
    Updates {
        /// Epoch day (days since Unix epoch)
        #[arg(long)]
        epoch_day: u64,
        /// Hour of day (0-23)
        #[arg(long)]
        hour_of_day: u32,
    },
    /// Validate a fixture by its ID
    Validate {
        /// Fixture ID
        #[arg(long)]
        fixture_id: u64,
        /// Expected timestamp
        #[arg(long)]
        timestamp: Option<u64>,
    },
    /// Batch-validate fixtures for a time window
    BatchValidate {
        /// Epoch day (days since Unix epoch)
        #[arg(long)]
        epoch_day: u64,
        /// Hour of day (0-23)
        #[arg(long)]
        hour_of_day: u32,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum OddsCommand {
    /// Get a snapshot of odds for a fixture
    Snapshot {
        /// Fixture ID
        #[arg(long)]
        fixture_id: u64,
    },
    /// Get odds updates for a fixture
    Updates {
        /// Fixture ID
        #[arg(long)]
        fixture_id: u64,
    },
    /// Get odds updates by time window
    UpdatesByTime {
        /// Epoch day (days since Unix epoch)
        #[arg(long)]
        epoch_day: u64,
        /// Hour of day (0-23)
        #[arg(long)]
        hour_of_day: u32,
        /// Interval in minutes
        #[arg(long)]
        interval: u32,
    },
    /// Validate an odds message
    Validate {
        /// Message ID
        #[arg(long)]
        message_id: String,
        /// Timestamp
        #[arg(long)]
        ts: u64,
    },
    /// Stream live odds updates via SSE
    Stream {
        /// Maximum number of events to receive
        #[arg(long)]
        limit: Option<u32>,
        /// Timeout in seconds
        #[arg(long)]
        timeout: Option<u64>,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum ScoresCommand {
    /// Get a snapshot of scores for a fixture
    Snapshot {
        /// Fixture ID
        #[arg(long)]
        fixture_id: u64,
        /// Timestamp to query scores as of
        #[arg(long)]
        as_of: Option<u64>,
    },
    /// Get scores updates for a fixture
    Updates {
        /// Fixture ID
        #[arg(long)]
        fixture_id: u64,
    },
    /// Get scores updates by time window
    UpdatesByTime {
        /// Epoch day (days since Unix epoch)
        #[arg(long)]
        epoch_day: u64,
        /// Hour of day (0-23)
        #[arg(long)]
        hour_of_day: u32,
        /// Interval in minutes
        #[arg(long)]
        interval: u32,
    },
    /// Get historical scores for a fixture
    Historical {
        /// Fixture ID
        #[arg(long)]
        fixture_id: u64,
    },
    /// Validate a scores stat
    Validate {
        /// Fixture ID
        #[arg(long)]
        fixture_id: u64,
        /// Sequence number
        #[arg(long)]
        seq: u64,
        /// Primary stat key
        #[arg(long)]
        stat_key: String,
        /// Secondary stat key (optional)
        #[arg(long)]
        stat_key2: Option<String>,
    },
    /// Stream live scores updates via SSE
    Stream {
        /// Maximum number of events to receive
        #[arg(long)]
        limit: Option<u32>,
        /// Timeout in seconds
        #[arg(long)]
        timeout: Option<u64>,
    },
}
