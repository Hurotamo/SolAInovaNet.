use sqlx::PgPool;

pub async fn insert_transaction(pool: &PgPool, user_id: i32, amount: f64) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO transactions (user_id, amount) VALUES ($1, $2)",
        user_id, amount
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_transactions(pool: &PgPool, user_id: i32) -> Result<Vec<(i32, f64)>, sqlx::Error> {
    let records = sqlx::query!(
        "SELECT id, amount FROM transactions WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(records.into_iter().map(|r| (r.id, r.amount)).collect())
}


