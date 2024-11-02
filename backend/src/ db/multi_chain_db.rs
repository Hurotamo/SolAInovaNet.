use sqlx::PgPool;

pub async fn store_chain_interaction(pool: &PgPool, chain_name: &str, interaction_data: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO multi_chain_interactions (chain_name, interaction_data) VALUES ($1, $2)",
        chain_name, interaction_data
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_chain_interactions(pool: &PgPool, chain_name: &str) -> Result<Vec<String>, sqlx::Error> {
    let records = sqlx::query!(
        "SELECT interaction_data FROM multi_chain_interactions WHERE chain_name = $1",
        chain_name
    )
    .fetch_all(pool)
    .await?;

    Ok(records.into_iter().map(|r| r.interaction_data).collect())
}


