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
use rust_decimal::prelude::*;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado {
    x : Vec<String>,
    y: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado2 {
    x : Vec<i32>,
    y: Vec<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado3 {
    x : Vec<String>,
    y: Vec<Decimal>
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
async fn give_accidents_state(anio: i32, pool: &sqlx::Pool<MySql>) -> Resultado {
        let y : Vec<(String, i32)> = make_query::<(String, i32)>(
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
    .unwrap();

    Resultado{
        x: y.iter().map(|a| a.0.clone()).collect(),
        y : y.iter().map(|a| a.1).collect()
    }
}

// fn give_alcolics_state(tabla :&str, pool :&sqlx::Pool<MySql>) -> Vec<>

async fn give_alcholic_state(anio: i32, pool: &sqlx::Pool<MySql>) -> Resultado {
        let y: Vec<(String, i32)> = make_query::<(String,i32)>(
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
    .unwrap();

    Resultado{
        x: y.iter().map(|a| a.0.clone()).collect(),
        y : y.iter().map(|a| a.1).collect()
    }

}

async fn give_type_accident(tyype: &str, pool: &sqlx::Pool<MySql>, anio : i32, state : i32) -> Vec<TypeAccidentByState>{
    make_query::<TypeAccidentByState>(format!("
    select NOM_ENTIDAD, tipo, cantidad from (select  COUNT(ID_ENTIDAD) AS
    cantidad,TIPACCID as tipo  from atus_anual where ANIO = {1} AND TIPACCID = '{0}' and ID_ENTIDAD = {2} group by ID_ENTIDAD, TIPACCID) AS res
    join tc_entidad using(ID_ENTIDAD);
    ", tyype.trim(), anio, state) ,pool).await.unwrap()

}
/*
select NOM_ENTIDAD, tipo, cantidad from (select ID_ENTIDAD, COUNT(ID_ENTIDAD) AS
    cantidad,TIPACCID as tipo  from atus_anual where ANIO = 2018 and ID_ENTIDAD = {2} group by ID_ENTIDAD, TIPACCID) AS res
    join tc_entidad using(ID_ENTIDAD);
*/

async fn give_accident_years(pool: &sqlx::Pool<MySql>) -> Resultado2 {
        let y: Vec<(i32, i32)> = make_query::<(i32, i32)>("SELECT ANIO, COUNT(ID) FROM atus_anual GROUP BY ANIO", pool).await.unwrap();
        Resultado2{
            x: y.iter().map(|a| a.0).collect(),
            y : y.iter().map(|a| a.1).collect()
        }
    }

async fn give_accidents_urbanos_rural(pool: &sqlx::Pool<MySql>) -> Resultado3{


        let mut y: Vec<(String, Decimal)> = make_query::<(String, Decimal)>("SELECT 'Zona Urbana' AS urbana, SUM(total) as accidentes_urbanos
        from(select count(id) as total from atus_anual where URBANA != 'Sin accidente en esta zona'  group by URBANA)a;
", pool).await.unwrap();

        let mut yo: Vec<(String, Decimal)> = make_query::<(String, Decimal)>("SELECT 'Zona Rural' as rural ,SUM(total) as accidentes_rurales
        from(select count(SUBURBANA) as total from atus_anual where SUBURBANA != 'Sin accidente en esta zona'  group by SUBURBANA) a", pool).await.unwrap();


        y.extend(yo);

        Resultado3{
            x: y.iter().map(|a| a.0.clone()).collect(),
            y: y.iter().map(|a| a.1).collect()
        }


    }







//SELECT SUM(total) as accidentes_urbanos  from(select count(id) as total from atus_anual where URBANA != 'Sin accidente en esta zona'  group by URBANA)a;

// SELECT SUM(total) from(select count(URBANA) as total from atus_anual where URBANA != 'Sin accidente en esta zona'  group by URBANA)a;
// SELECT SUM(total) from(select count(id) as total from atus_anual where SUBURBANA != 'Sin accidente en esta zona'  group by SUBURBANA)a;



// advertecia algunos id tienes que ponerlos como 09  como en el ID_ENTIDAD depende de comos e creo el csv (en mi caso en la laptop se creo sin el 0 y en la pc se creo 01)

async fn query1(Path(anio): Path<i32>, State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_accidents_state(anio, &state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn query2(Path(anio): Path<i32>, State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
        println!("{}", anio);
        let msg = give_alcholic_state(anio, &state).await;
        let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
        response
    }
async fn query3(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {

        let msg = give_accident_years(&state).await;
        let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
        response
    }

async fn query4(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {

        let msg = give_accidents_urbanos_rural(&state).await;
        println!("{:?}", msg);

        let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("*"));
        response.headers_mut().append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
        response
    }


async fn page() -> impl IntoResponse {
    let html = std::fs::read_to_string("index.html").unwrap();

    let mut response = Html(html).into_response();
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_CREDENTIALS, HeaderValue::from_static("*"));
    response.headers_mut().append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn styles() -> impl IntoResponse {
    let css = std::fs::read_to_string("output.css").unwrap();

    let mut response = (StatusCode::OK, css).into_response();
    response.headers_mut().append(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("text/css"));

    response
}

#[tokio::main]
async fn main() {
    let state = Arc::new(MySqlPool::connect("mariadb://root:Pinky01*@127.0.0.1/accidents").await.unwrap());
    let router = Router::new()
        .route("/styles.css", get(styles))
        .route("/query1/:anio", get(query1))
        .route("/query2/:anio", get(query2))
        .route("/query3/", get(query3))
        .route("/query4/", get(query4))
        .route("/page", get(page))
        .with_state(state);

    Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
