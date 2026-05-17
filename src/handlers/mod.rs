use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    Json,
};
use tracing::{info, warn};

use crate::state::SharedState;

// GET /api/rounds/current
pub async fn current_round(State(state): State<SharedState>) -> impl IntoResponse {
    let round = state.latest_round.read().await;
    Json(round.clone())
}

// GET /ws/live
pub async fn ws_live(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: SharedState) {
    // each ws connection gets its own Receiver from the broadcast channel
    let mut rx = state.round_tx.subscribe();

    info!("WebSocket client connected");

    loop {
        match rx.recv().await {
            Ok(round) => {
                let json = match serde_json::to_string(&round) {
                    Ok(j) => j,
                    Err(e) => {
                        warn!("Failed to serialize round: {}", e);
                        continue;
                    }
                };
                if socket.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                warn!("WebSocket client lagged, dropped {} events", n);
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                break;
            }
        }
    }

    info!("WebSocket client disconnected");
}
