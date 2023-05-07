use anyhow::{anyhow, Error, Result};
use leptos::*;
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql};
use std::any::Any;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tokio::time::error::Elapsed;

async fn conection() -> MySqlPool {
    MySqlPool::connect("msyql://root:1234@localhost/Accidents")
        .await
        .unwrap()
}

pub async fn make_query<T>(query: impl AsRef<str>, connection: &sqlx::Pool<MySql>) -> Result<Vec<T>>
where
    for<'a> T: sqlx::FromRow<'a, sqlx::mysql::MySqlRow> + Send + Unpin,
{
    let result: Vec<T> = sqlx::query_as::<_, T>(query.as_ref())
        .fetch_all(connection)
        .await?;

    if result.is_empty() {
        Err(anyhow!("No se encontró ningún dato"))
    } else {
        Ok(result)
    }
}

pub async fn make_query_expect_empty<T>(
    query: impl AsRef<str>,
    connection: &sqlx::Pool<MySql>,
) -> Result<Vec<T>>
where
    for<'a> T: sqlx::FromRow<'a, sqlx::mysql::MySqlRow> + Send + Unpin,
{
    let result: Vec<T> = sqlx::query_as::<_, T>(query.as_ref())
        .fetch_all(connection)
        .await?;

    if !result.is_empty() {
        Err(anyhow!("Se encontró uno o más datos"))
    } else {
        Ok(result)
    }
}

fn main() {
    let pool = conection();

    println!("hola");
}
