use color_eyre::Result;

use crate::api::ApiClient;
use crate::cli::ScoresCommand;
use crate::output::print_json;
use crate::stream;
use crate::types::{Scores, ScoresStatValidation};

pub async fn handle(client: &ApiClient, cmd: ScoresCommand, raw: bool) -> Result<()> {
    match cmd {
        ScoresCommand::Snapshot { fixture_id, as_of } => {
            let data = snapshot(client, fixture_id, as_of).await?;
            print_json(&data, raw)?;
        }
        ScoresCommand::Updates { fixture_id } => {
            let data = updates(client, fixture_id).await?;
            print_json(&data, raw)?;
        }
        ScoresCommand::UpdatesByTime {
            epoch_day,
            hour_of_day,
            interval,
        } => {
            let data = updates_by_time(client, epoch_day, hour_of_day, interval).await?;
            print_json(&data, raw)?;
        }
        ScoresCommand::Historical { fixture_id } => {
            let data = historical(client, fixture_id).await?;
            print_json(&data, raw)?;
        }
        ScoresCommand::Validate {
            fixture_id,
            seq,
            stat_key,
            stat_key2,
        } => {
            let data = validate(client, fixture_id, seq, &stat_key, stat_key2.as_deref()).await?;
            print_json(&data, raw)?;
        }
        ScoresCommand::Stream { limit, timeout } => {
            let data = stream_scores(client, limit, timeout).await?;
            print_json(&data, raw)?;
        }
    }
    Ok(())
}

pub async fn snapshot(
    client: &ApiClient,
    fixture_id: u64,
    as_of: Option<u64>,
) -> Result<Vec<Scores>> {
    let path = match as_of {
        Some(ts) => format!("/api/scores/snapshot/{fixture_id}?asOf={ts}"),
        None => format!("/api/scores/snapshot/{fixture_id}"),
    };
    client.get_json(&path).await
}

pub async fn updates(client: &ApiClient, fixture_id: u64) -> Result<Vec<Scores>> {
    let path = format!("/api/scores/updates/{fixture_id}");
    client.get_json(&path).await
}

pub async fn updates_by_time(
    client: &ApiClient,
    epoch_day: u64,
    hour_of_day: u32,
    interval: u32,
) -> Result<Vec<Scores>> {
    let path = format!("/api/scores/updates/{epoch_day}/{hour_of_day}/{interval}");
    client.get_json(&path).await
}

pub async fn historical(client: &ApiClient, fixture_id: u64) -> Result<Vec<Scores>> {
    let path = format!("/api/scores/historical/{fixture_id}");
    client.get_json(&path).await
}

pub async fn validate(
    client: &ApiClient,
    fixture_id: u64,
    seq: u64,
    stat_key: &str,
    stat_key2: Option<&str>,
) -> Result<ScoresStatValidation> {
    let mut path = format!(
        "/api/scores/stat-validation?fixtureId={fixture_id}&seq={seq}&statKey={stat_key}"
    );
    if let Some(key2) = stat_key2 {
        path.push_str(&format!("&statKey2={key2}"));
    }
    client.get_json(&path).await
}

pub async fn stream_scores(
    client: &ApiClient,
    limit: Option<u32>,
    timeout_secs: Option<u64>,
) -> Result<Vec<crate::types::SseMessage>> {
    let req = client.get("/api/scores/stream");
    let response = req.send().await?;
    stream::read_sse_stream(response, limit, timeout_secs).await
}
