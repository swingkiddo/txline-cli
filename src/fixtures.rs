use color_eyre::Result;

use crate::api::ApiClient;
use crate::cli::FixturesCommand;
use crate::types::{Fixture, FixtureValidation};

fn print_json<T: serde::Serialize>(data: &T, raw: bool) -> Result<()> {
    if raw {
        println!("{}", serde_json::to_string(data)?);
    } else {
        println!("{}", serde_json::to_string_pretty(data)?);
    }
    Ok(())
}

pub async fn handle(client: &ApiClient, cmd: FixturesCommand, raw: bool) -> Result<()> {
    match cmd {
        FixturesCommand::Snapshot { competition_id } => {
            let data = snapshot(client, competition_id).await?;
            print_json(&data, raw)?;
        }
        FixturesCommand::Updates {
            epoch_day,
            hour_of_day,
        } => {
            let data = updates(client, epoch_day, hour_of_day).await?;
            print_json(&data, raw)?;
        }
        FixturesCommand::Validate {
            fixture_id,
            timestamp,
        } => {
            let data = validate(client, fixture_id, timestamp).await?;
            print_json(&data, raw)?;
        }
        FixturesCommand::BatchValidate {
            epoch_day,
            hour_of_day,
        } => {
            let data = batch_validate(client, epoch_day, hour_of_day).await?;
            print_json(&data, raw)?;
        }
    }
    Ok(())
}

pub async fn snapshot(
    client: &ApiClient,
    competition_id: Option<u64>,
) -> Result<Vec<Fixture>> {
    let path = match competition_id {
        Some(id) => format!("/api/fixtures/snapshot?competitionId={id}"),
        None => "/api/fixtures/snapshot".to_string(),
    };
    client.get_json(&path).await
}

pub async fn updates(
    client: &ApiClient,
    epoch_day: u64,
    hour_of_day: u32,
) -> Result<Vec<Fixture>> {
    let path = format!("/api/fixtures/updates/{epoch_day}/{hour_of_day}");
    client.get_json(&path).await
}

pub async fn validate(
    client: &ApiClient,
    fixture_id: u64,
    timestamp: Option<u64>,
) -> Result<FixtureValidation> {
    let path = match timestamp {
        Some(ts) => format!("/api/fixtures/validation?fixtureId={fixture_id}&timestamp={ts}"),
        None => format!("/api/fixtures/validation?fixtureId={fixture_id}"),
    };
    client.get_json(&path).await
}

pub async fn batch_validate(
    client: &ApiClient,
    epoch_day: u64,
    hour_of_day: u32,
) -> Result<Vec<FixtureValidation>> {
    let path = format!("/api/fixtures/batch-validation?epochDay={epoch_day}&hourOfDay={hour_of_day}");
    client.get_json(&path).await
}
