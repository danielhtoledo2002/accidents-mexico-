use anyhow::{anyhow, Error, Result};
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql};
use std::time::Duration;
use tokio::time::error::Elapsed;

#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct StateAccidents {
    ID_ENTIDAD: String,
    numero_accidentes:i32,
}

async fn conection() -> MySqlPool {
    MySqlPool::connect("mariadb://root:1234@localhost/Accidents")
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


/*
advertecia algunos id tienes que ponerlos como 09 como en el ID_ENTIDAD
*/


#[tokio::main]
async fn main() {
    let mut pool = MySqlPool::connect("mariadb://root:1234@localhost/Accidents")
        .await
        .unwrap();
      
    let query = make_query::<StateAccidents>("select ID_ENTIDAD, COUNT(ID_ENTIDAD) as numero_accidentes from accidentes_2020 GROUP BY ID_ENTIDAD HAVING COUNT(ID_ENTIDAD) > 1 ",
        &mut pool,
    )
    .await
    .unwrap();
    println!("{:?}", query);

    
}
