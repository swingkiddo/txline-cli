use clap::CommandFactory;
use clap::Parser;
use color_eyre::Result;
use tracing_subscriber::EnvFilter;

mod api;
mod auth;
mod cli;
mod config;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = cli::Cli::parse();

    let mut config = config::Config::load()?;
    if let Some(ref network) = cli.network {
        config.network = network
            .parse::<config::Network>()
            .unwrap_or_else(|_| {
                eprintln!("Invalid network: {network}. Using devnet.");
                config::Network::Devnet
            });
        config.api_host = config.network.api_host().to_string();
    }

    let client = api::ApiClient::new(config);

    match cli.command {
        cli::Commands::Auth { command } => match command {
            cli::AuthCommand::Guest => {
                auth::guest_auth(&client).await?;
            }
            cli::AuthCommand::Activate { tx_sig, keypair } => {
                auth::activate_token(&client, &tx_sig, &keypair, &[]).await?;
            }
        },
        cli::Commands::Subscribe { .. } => {
            tracing::info!("Subscribe command — not yet implemented (Phase 2)");
            println!("Subscribe: not yet implemented (Phase 2)");
        }
        cli::Commands::Fixtures { .. } => {
            tracing::info!("Fixtures command — not yet implemented (Phase 3)");
            println!("Fixtures: not yet implemented (Phase 3)");
        }
        cli::Commands::Odds { .. } => {
            tracing::info!("Odds command — not yet implemented (Phase 3)");
            println!("Odds: not yet implemented (Phase 3)");
        }
        cli::Commands::Scores { .. } => {
            tracing::info!("Scores command — not yet implemented (Phase 3)");
            println!("Scores: not yet implemented (Phase 3)");
        }
        cli::Commands::Completions { shell } => {
            let mut cmd = cli::Cli::command();
            clap_complete::generate(shell, &mut cmd, "txodds", &mut std::io::stdout());
        }
    }

    Ok(())
}
