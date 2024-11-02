use sqlx::PgPool;

pub async fn insert_ai_node_data(pool: &PgPool, data: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO ai_nodes (data) VALUES ($1)",
        data
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_ai_node_data(pool: &PgPool, id: i32) -> Result<String, sqlx::Error> {
    let record = sqlx::query!(
        "SELECT data FROM ai_nodes WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(record.data)
}


