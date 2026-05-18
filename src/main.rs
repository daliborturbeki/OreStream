mod handlers;
mod state;
mod config;

use arete_sdk::prelude::*;
use axum::{routing::get, Router};
use ore_stack::{OreRound, OreStreamStack};
use tracing::info;
use crate::config::Config;

fn print_round(round: &OreRound) {
    info!(
        round_id = round.id.round_id.unwrap_or(0),
        motherlode = ?round.state.motherlode,
        total_deployed = ?round.state.total_deployed,
        deploy_count = ?round.metrics.deploy_count,
        "OreRound update"
    );
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let app_state = state::AppState::new();

    info!("Connecting to Ore stream...");

    let a4 = Arete::<OreStreamStack>::builder()
        .api_key(&config.arete_api_key)
        .connect()
        .await?;

    info!("Connected!");

    let stream_state = app_state.clone();
    tokio::spawn(async move {
        let mut stream = a4.views.ore_round.latest().listen();
        while let Some(round) = stream.next().await {
            if round.id.round_id.is_none() {
                continue;
            }
            print_round(&round);
            *stream_state.latest_round.write().await = Some(round.clone());
            let _ = stream_state.round_tx.send(round);
        }
    });

    let app = Router::new()
        .route("/api/rounds/current", get(handlers::current_round))
        .route("/ws/live", get(handlers::ws_live))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;
    info!("HTTP server listening on http://0.0.0.0:{}", config.port);

    axum::serve(listener, app).await?;

    Ok(())
}
