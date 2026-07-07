use clap::Parser;
use color_eyre::Result;
use tracing_subscriber::EnvFilter;

mod api;
mod auth;
mod cli;
mod config;
mod dispatch;
mod fixtures;
mod odds;
mod output;
mod scores;
mod stream;
mod subscribe;
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

    let network = config.network.clone();
    let client = api::ApiClient::new(config);

    dispatch::dispatch_command(&cli, &client, &network).await?;

    Ok(())
}
