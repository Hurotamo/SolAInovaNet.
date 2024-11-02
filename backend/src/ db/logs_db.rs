use sqlx::PgPool;

pub async fn log_event(pool: &PgPool, event: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO logs (event) VALUES ($1)",
        event
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_recent_logs(pool: &PgPool, limit: i32) -> Result<Vec<String>, sqlx::Error> {
    let records = sqlx::query!(
        "SELECT event FROM logs ORDER BY created_at DESC LIMIT $1",
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(records.into_iter().map(|r| r.event).collect())
}


