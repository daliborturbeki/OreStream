use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use ore_stack::OreRound;

pub struct AppState {
    pub latest_round: RwLock<Option<OreRound>>,
    pub round_tx: broadcast::Sender<OreRound>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        let (round_tx, _) = broadcast::channel(256);
        Arc::new(Self {
            latest_round: RwLock::new(None),
            round_tx,
        })
    }
}

pub type SharedState = Arc<AppState>;
