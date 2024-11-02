use sqlx::PgPool;

pub async fn record_metric(pool: &PgPool, metric_name: &str, value: f64) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO analytics (metric_name, value) VALUES ($1, $2)",
        metric_name, value
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_metrics(pool: &PgPool, metric_name: &str) -> Result<Vec<f64>, sqlx::Error> {
    let records = sqlx::query!(
        "SELECT value FROM analytics WHERE metric_name = $1",
        metric_name
    )
    .fetch_all(pool)
    .await?;

    Ok(records.into_iter().map(|r| r.value).collect())
}


