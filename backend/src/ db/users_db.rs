use sqlx::PgPool;

pub async fn create_user(pool: &PgPool, username: &str, email: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (username, email) VALUES ($1, $2)",
        username, email
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user_by_id(pool: &PgPool, user_id: i32) -> Result<(String, String), sqlx::Error> {
    let record = sqlx::query!(
        "SELECT username, email FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok((record.username, record.email))
}


