use sqlx::PgPool;
pub async fn add_wallet(pool: &PgPool, user_id: i32, wallet_address: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO wallets (user_id, wallet_address) VALUES ($1, $2)",
        user_id, wallet_address
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn get_wallets(pool: &PgPool, user_id: i32) -> Result<Vec<String>, sqlx::Error> {
    let records = sqlx::query!(
        "SELECT wallet_address FROM wallets WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;
    Ok(records.into_iter().map(|r| r.wallet_address).collect())
}
