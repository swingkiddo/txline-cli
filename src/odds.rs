use color_eyre::Result;

use crate::api::ApiClient;
use crate::cli::OddsCommand;
use crate::output::print_json;
use crate::stream;
use crate::types::{OddsPayload, OddsValidation};
use crate::validation;

pub async fn handle(client: &ApiClient, cmd: OddsCommand, raw: bool) -> Result<()> {
    match cmd {
        OddsCommand::Snapshot { fixture_id } => {
            let data = snapshot(client, fixture_id).await?;
            print_json(&data, raw)?;
        }
        OddsCommand::Updates { fixture_id } => {
            let data = updates(client, fixture_id).await?;
            print_json(&data, raw)?;
        }
        OddsCommand::UpdatesByTime {
            epoch_day,
            hour_of_day,
            interval,
        } => {
            let data = updates_by_time(client, epoch_day, hour_of_day, interval).await?;
            print_json(&data, raw)?;
        }
        OddsCommand::Validate { message_id, ts } => {
            let data = validate(client, &message_id, ts).await?;
            let leaf = validation::hash_odds(&data.odds);
            let sub_tree_valid = validation::verify_merkle_proof(
                &leaf,
                &data.sub_tree_proof,
                &data.summary.odds_sub_tree_root,
            );
            let summary_leaf = validation::hash_odds_summary(&data.summary);
            let main_tree_valid = validation::verify_merkle_proof(
                &summary_leaf,
                &data.main_tree_proof,
                &[],
            );
            let result = crate::types::ValidationResult {
                data,
                sub_tree_valid,
                main_tree_valid,
            };
            print_json(&result, raw)?;
        }
        OddsCommand::Stream { limit, timeout } => {
            let data = stream_odds(client, limit, timeout).await?;
            print_json(&data, raw)?;
        }
    }
    Ok(())
}

pub async fn snapshot(client: &ApiClient, fixture_id: u64) -> Result<Vec<OddsPayload>> {
    let path = format!("/api/odds/snapshot/{fixture_id}");
    client.get_json(&path).await
}

pub async fn updates(client: &ApiClient, fixture_id: u64) -> Result<Vec<OddsPayload>> {
    let path = format!("/api/odds/updates/{fixture_id}");
    client.get_json(&path).await
}

pub async fn updates_by_time(
    client: &ApiClient,
    epoch_day: u64,
    hour_of_day: u32,
    interval: u32,
) -> Result<Vec<OddsPayload>> {
    let path = format!("/api/odds/updates/{epoch_day}/{hour_of_day}/{interval}");
    client.get_json(&path).await
}

pub async fn validate(
    client: &ApiClient,
    message_id: &str,
    ts: u64,
) -> Result<OddsValidation> {
    let path = format!("/api/odds/validation?messageId={message_id}&ts={ts}");
    client.get_json(&path).await
}

pub async fn stream_odds(
    client: &ApiClient,
    limit: Option<u32>,
    timeout_secs: Option<u64>,
) -> Result<Vec<crate::types::SseMessage>> {
    let req = client.get("/api/odds/stream");
    let response = req.send().await?;
    stream::read_sse_stream(response, limit, timeout_secs).await
}
