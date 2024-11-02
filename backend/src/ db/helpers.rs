use sqlx::PgPool;
use std::time::Duration;

pub async fn run_transaction<F, T>(pool: &PgPool, f: F) -> Result<T, sqlx::Error>
where
    F: FnOnce(&mut sqlx::Transaction<'_, sqlx::Postgres>) -> T,
{
    let mut transaction = pool.begin().await?;
    let result = f(&mut transaction);
    transaction.commit().await?;
    Ok(result)
}

pub fn retry<F, T>(mut attempt: F, retries: usize, delay: Duration) -> T
where
    F: FnMut() -> Result<T, ()>,
{
    for _ in 0..retries {
        match attempt() {
            Ok(value) => return value,
            Err(_) => std::thread::sleep(delay),
        }
    }
    panic!("Operation failed after retries");
}


