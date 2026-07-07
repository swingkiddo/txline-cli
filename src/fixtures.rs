use color_eyre::Result;

use crate::api::ApiClient;
use crate::cli::FixturesCommand;
use crate::output::print_json;
use crate::types::{Fixture, FixtureValidation};
use crate::validation;

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
            let leaf = validation::hash_fixture(&data.snapshot);
            let sub_tree_valid = validation::verify_merkle_proof(
                &leaf,
                &data.sub_tree_proof,
                &data.summary.update_sub_tree_root,
            );
            let summary_leaf = validation::hash_fixture_summary(&data.summary);
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
