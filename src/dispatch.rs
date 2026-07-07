use clap::CommandFactory;
use color_eyre::Result;

use crate::api::ApiClient;
use crate::auth;
use crate::cli::{Cli, Commands};
use crate::config::Network;
use crate::fixtures;
use crate::odds;
use crate::scores;
use crate::subscribe;

pub async fn dispatch_command(cli: &Cli, client: &ApiClient, network: &Network) -> Result<()> {
    match &cli.command {
        Commands::Auth { command } => match command {
            crate::cli::AuthCommand::Guest => {
                auth::guest_auth(client).await?;
            }
            crate::cli::AuthCommand::Activate { tx_sig, keypair } => {
                auth::activate_token(client, tx_sig, keypair, &[]).await?;
            }
        },
        Commands::Subscribe {
            keypair,
            service_level,
            weeks,
            rpc,
        } => {
            let tx_sig =
                subscribe::subscribe_onchain(keypair, network, *service_level, *weeks, rpc).await?;
            if cli.raw {
                println!("{tx_sig}");
            } else {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({ "txSig": tx_sig }))?
                );
            }
        }
        Commands::Fixtures { command } => {
            fixtures::handle(client, command.clone(), cli.raw).await?;
        }
        Commands::Odds { command } => {
            odds::handle(client, command.clone(), cli.raw).await?;
        }
        Commands::Scores { command } => {
            scores::handle(client, command.clone(), cli.raw).await?;
        }
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(*shell, &mut cmd, "txodds", &mut std::io::stdout());
        }
    }
    Ok(())
}
