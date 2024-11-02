use sqlx::{Pool, Postgres};
use std::env;

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment variables");
    Pool::<Postgres>::connect(&database_url).await
}


