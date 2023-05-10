use anyhow::{anyhow, Error, Result};
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql, Pool};
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use axum::{Router, Server};
use axum::extract::{Path, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::header::{ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use tokio::time::error::Elapsed;


#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct AlcholStates {
    NOM_ENTIDAD: String,
    borrachos: i32,
}

// struct to obtain the data from the table in mysql
#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
struct StateAccidentsSql {
    #[sqlx(rename = "NOM_ENTIDAD")]
    nombre_entidad: String,
    numero_accidentes: i32,
}

//struct to obtain number for each type of accident by state

#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct TypeAccidentByState {
    NOM_ENTIDAD: String,
    tipo: String,
    cantidad: i32,
}



// function to fill a generic struct

async fn make_query<T>(query: impl AsRef<str>, connection: &sqlx::Pool<MySql>) -> Result<Vec<T>>
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

// Give me all the accidents that happen in a state in the year without any filter
async fn give_accidents_state(anio: i32, pool: &sqlx::Pool<MySql>) -> Vec<StateAccidentsSql> {
    make_query::<StateAccidentsSql>(
        format!(
            "SELECT NOM_ENTIDAD, numero_accidentes FROM (
SELECT
 ID_ENTIDAD, count(ID) as numero_accidentes 
 FROM atus_anual
  WHERE ANIO = {} 
  GROUP BY ID_ENTIDAD
) AS res
JOIN tc_entidad USING(ID_ENTIDAD);  ",
            anio
        ),
        pool,
    ).await
    .unwrap()
}

// fn give_alcolics_state(tabla :&str, pool :&sqlx::Pool<MySql>) -> Vec<>

async fn give_alcholic_state(anio: i32, pool: &sqlx::Pool<MySql>) -> Vec<AlcholStates> {
    make_query::<AlcholStates>(
        format!(
            "SELECT NOM_ENTIDAD, borrachos FROM (
                SELECT
                 ID_ENTIDAD, count(ID) as borrachos 
                 FROM atus_anual
                  WHERE ANIO = {} 
                  AND ALIENTO = 'si' 
                  GROUP BY ID_ENTIDAD, ALIENTO
                ) AS res
    JOIN tc_entidad USING(ID_ENTIDAD);",
            anio
        ),
        pool,
    ).await
    .unwrap()
}

async fn give_type_accident(tyype: &str, pool: &sqlx::Pool<MySql>, anio : i32, state : i32) -> Vec<TypeAccidentByState>{
    make_query::<TypeAccidentByState>(format!("
    select NOM_ENTIDAD, tipo, cantidad from (select ID_ENTIDAD, COUNT(ID_ENTIDAD) AS
    cantidad,TIPACCID as tipo  from atus_anual where ANIO = {1} AND TIPACCID = '{0}' and ID_ENTIDAD = {2} group by ID_ENTIDAD, TIPACCID) AS res
    join tc_entidad using(ID_ENTIDAD);
    ", tyype.trim(), anio, state) ,pool).await.unwrap()

}


// advertecia algunos id tienes que ponerlos como 09  como en el ID_ENTIDAD depende de comos e creo el csv (en mi caso en la laptop se creo sin el 0 y en la pc se creo 01)

async fn query1(Path(anio): Path<i32>, State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    println!("{}", anio);
    let msg = give_accidents_state(anio, &state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn page() -> impl IntoResponse {
    let html = include_str!("../index.html");

    let mut response = Html(html).into_response();
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

#[tokio::main]
async fn main() {
    let state = Arc::new(MySqlPool::connect("mariadb://root:Pinky01*@localhost/accidents").await.unwrap());
    let router = Router::new()
        .route("/query1/:anio", get(query1))
        .route("/page", get(page))
        .with_state(state);

    Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
