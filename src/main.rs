use arete_sdk::prelude::*;
use ore_stack::{OreRound, OreStreamStack, OreTreasury};
use tracing::info;

fn print_round(round: &OreRound) {
    info!(
        round_id = round.id.round_id.unwrap_or(0),
        motherlode = ?round.state.motherlode,
        total_deployed = ?round.state.total_deployed,
        deploy_count = ?round.metrics.deploy_count,
        "OreRound update"
    );
}

fn print_treasury(treasury: &OreTreasury) {
    info!(
        address = ?treasury.id.address,
        balance = ?treasury.state.balance,
        total_refined = ?treasury.state.total_refined,
        "OreTreasury update"
    );
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let api_key = std::env::var("ARETE_API_KEY")
        .expect("ARETE_API_KEY must be set in .env or environment");

    info!("Connecting to Ore stream...");

    let a4 = Arete::<OreStreamStack>::builder()
        .api_key(&api_key)
        .connect()
        .await?;

    info!("Connected! Streaming OreRound and OreTreasury...");

    let round_view = a4.views.ore_round.latest();
    let treasury_view = a4.views.ore_treasury.list();

    let round_handle = tokio::spawn(async move {
        let mut stream = round_view.listen();
        while let Some(round) = stream.next().await {
            if round.id.round_id.is_some() {
                print_round(&round);
            }
        }
    });

    let treasury_handle = tokio::spawn(async move {
        let mut stream = treasury_view.listen();
        while let Some(treasury) = stream.next().await {
            if treasury.id.address.is_some() {
                print_treasury(&treasury);
            }
        }
    });

    let _ = tokio::join!(round_handle, treasury_handle);

    Ok(())
}