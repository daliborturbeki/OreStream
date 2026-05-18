use sqlx::PgPool;
use ore_stack::OreRound;

pub async fn insert_round(pool: &PgPool, round: &OreRound) -> anyhow::Result<()> {
    sqlx::query!(
        "INSERT INTO rounds (round_id, motherlode, total_deployed, deploy_count)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (round_id) DO UPDATE SET
             motherlode = COALESCE(EXCLUDED.motherlode, rounds.motherlode),
             total_deployed = COALESCE(EXCLUDED.total_deployed, rounds.total_deployed),
             deploy_count = COALESCE(EXCLUDED.deploy_count, rounds.deploy_count),
             recorded_at = NOW()",
        round.id.round_id.map(|v| v as i64),
        round.state.motherlode.flatten(),
        round.state.total_deployed.flatten(),
        round.metrics.deploy_count.flatten().map(|v| v as i64),
    )
    .execute(pool)
    .await?;
    Ok(())
}