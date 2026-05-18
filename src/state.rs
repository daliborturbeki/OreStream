use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use ore_stack::OreRound;
use sqlx::PgPool;

pub struct AppState {
    pub db: PgPool,
    pub latest_round: RwLock<Option<OreRound>>,
    pub round_tx: broadcast::Sender<OreRound>,
}

impl AppState {
    pub fn new(db: PgPool) -> Arc<Self> {
        let (round_tx, _) = broadcast::channel(256);
        Arc::new(Self {
            db,
            latest_round: RwLock::new(None),
            round_tx,
        })
    }
}

pub type SharedState = Arc<AppState>;
