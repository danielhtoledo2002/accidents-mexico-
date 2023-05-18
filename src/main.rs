use anyhow::{anyhow, Error, Result};
use axum::extract::{Path, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN,
};
use axum::http::{HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Router, Server};
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql, Pool};
use std::io::Write;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::error::Elapsed;

//struct to obtain number for each type of accident by state

#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct TypeAccidentByState {
    NOM_ENTIDAD: String,
    tipo: String,
    cantidad: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado {
    x: Vec<String>,
    y: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado2 {
    x: Vec<i32>,
    y: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado3 {
    x: Vec<String>,
    y: Vec<Decimal>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado4 {
    x: Vec<String>,
    y: Vec<i32>,
    z: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado5 {
    x: Vec<String>,
    y: Vec<String>,
    z: Vec<i32>,
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

// Give the total accidents that happened by state in year.

async fn give_accidents_state(anio: i32, pool: &sqlx::Pool<MySql>) -> Resultado {
    let y: Vec<(String, i32)> = make_query::<(String, i32)>(
        format!(
            "SELECT NOM_ENTIDAD, numero_accidentes FROM (
             SELECT ID_ENTIDAD, count(ID) as numero_accidentes
             FROM atus_anual
              WHERE ANIO = {}
              GROUP BY ID_ENTIDAD
            ) AS res
            JOIN tc_entidad USING(ID_ENTIDAD);  ",
            anio
        ),
        pool,
    )
    .await
    .unwrap();

    Resultado {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// Give the number of people who committed an accident when they where drunk in a year
// you can select the year

async fn give_alcholic_state(anio: i32, pool: &sqlx::Pool<MySql>) -> Resultado {
    let y: Vec<(String, i32)> = make_query::<(String, i32)>(
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
    )
    .await
    .unwrap();

    Resultado {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// Give the total of accident by state (toca crear un boton para elegir el año, el estado y el tipo de acidente)

async fn give_type_accident(
    tyype: &str,
    anio: i32,
    state: &str,
    pool: &sqlx::Pool<MySql>,
) -> Resultado5 {
    let y : Vec<(String, String, i32)>  = make_query::<(String, String, i32)>(format!("
   select NOM_ENTIDAD, tipo, cantidad from
    (select  id_entidad, COUNT(ID_ENTIDAD) AS  cantidad,TIPACCID as tipo
    from atus_anual where ANIO = {1} AND TIPACCID = '{0}'
    group by ID_ENTIDAD, TIPACCID) as res join tc_entidad using(ID_ENTIDAD) where tc_entidad.NOM_ENTIDAD = '{2}';
    ", tyype.trim(), anio, state) ,pool).await.unwrap();

    Resultado5 {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1.clone()).collect(),
        z: y.iter().map(|a| a.2).collect(),
    }
    /*
    TYPE ACCIDENTS
    | Colisión con vehículo automotor         |
    | Colisión con peatón (atropellamiento)   |
    | Colisión con objeto fijo                |
    | Colisión con motocicleta                |
    | Salida del camino                       |
    | Colisión con ferrocarril                |
    | Colisión con ciclista                   |
    | Volcadura                               |
    | Colisión con animal                     |
    | Otro                                    |
    | Caída de pasajero                       |
    | Certificado cero                        |
    | Inedio                                  |

    */
}

// Number of accident for each year

async fn give_accident_years(pool: &sqlx::Pool<MySql>) -> Resultado2 {
    let y: Vec<(i32, i32)> =
        make_query::<(i32, i32)>("SELECT ANIO, COUNT(ID) FROM atus_anual GROUP BY ANIO", pool)
            .await
            .unwrap();
    Resultado2 {
        x: y.iter().map(|a| a.0).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// give the number of accident by zone
async fn give_accidents_urbanos_rural(pool: &sqlx::Pool<MySql>) -> Resultado3 {
    let mut y: Vec<(String, Decimal)> = make_query::<(String, Decimal)>("SELECT 'Zona Urbana' AS urbana, SUM(total) as accidentes_urbanos
        from(select count(id) as total from atus_anual where URBANA != 'Sin accidente en esta zona'  group by URBANA)a;
", pool).await.unwrap();

    let mut yo: Vec<(String, Decimal)> = make_query::<(String, Decimal)>("SELECT 'Zona Rural' as rural ,SUM(total) as accidentes_rurales
        from(select count(SUBURBANA) as total from atus_anual where SUBURBANA != 'Sin accidente en esta zona'  group by SUBURBANA) a", pool).await.unwrap();

    y.extend(yo);

    Resultado3 {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// give the day and hour from accidedents

async fn give_day_hour_accidents(pool: &sqlx::Pool<MySql>) -> Resultado4 {
    let mut y: Vec<(String, i32, i32)> = make_query::<(String, i32, i32)>(
        "Select diasemana, id_hora, count(diasemana) as num_accidentes
        From atus_anual where diasemana!= 'certificado cero' group by diasemana,id_hora;
        ",
        pool,
    )
    .await
    .unwrap();

    Resultado4 {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
        z: y.iter().map(|a| a.2).collect(),
    }
}

// give the total of accidents that happened 0 -24 h
async fn give_total_accidents_hours(pool: &sqlx::Pool<MySql>) -> Resultado2 {
    let mut y: Vec<(i32, i32)> = make_query::<(i32, i32)>(
        "select id_hora, count(id) from atus_anual group by id_hora;",
        pool,
    )
    .await
    .unwrap();
    Resultado2 {
        x: y.iter().map(|a| a.0).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// give the total of accidents by each accident

async fn give_types_accidents(pool: &sqlx::Pool<MySql>) -> Resultado {
    let mut y: Vec<(String, i32)> = make_query::<(String, i32)>(
        "select tipaccid, count(id) from atus_anual group by tipaccid;
",
        pool,
    )
    .await
    .unwrap();
    Resultado {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// men vs women
async fn give_man_woman_accidents(pool: &sqlx::Pool<MySql>) -> Resultado {
    let mut y: Vec<(String, i32)> = make_query::<(String, i32)>(
        "Select sexo, count(sexo) as num_accidentes
        From atus_anual where sexo ='Hombre' or sexo= 'Mujer' Group by sexo;",
        pool,
    )
    .await
    .unwrap();

    Resultado {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// day number of accidents

async fn give_day_number_accidents(pool: &sqlx::Pool<MySql>) -> Resultado {
    let mut y: Vec<(String, i32)> = make_query::<(String, i32)>(
        "Select diasemana, count(diasemana) as num_accidentes
        from atus_anual where diasemana != 'Certificado cero'
        Group by diasemana;",
        pool,
    )
    .await
    .unwrap();

    Resultado {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

//SELECT SUM(total) as accidentes_urbanos  from(select count(id) as total from atus_anual where URBANA != 'Sin accidente en esta zona'  group by URBANA)a;

async fn query1(Path(anio): Path<i32>, State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_accidents_state(anio, &state).await;

    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn query2(Path(anio): Path<i32>, State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_alcholic_state(anio, &state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}
async fn query3(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_accident_years(&state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn query4(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_accidents_urbanos_rural(&state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn query5(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_total_accidents_hours(&state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn query6(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_types_accidents(&state).await;
    println!("{:?}", msg);
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}
async fn query7(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_day_hour_accidents(&state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn query8(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_man_woman_accidents(&state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn query9(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_day_number_accidents(&state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn query10(
    Path((anio, tyype, stat)): Path<(i32, &str, &str)>,
    State(state): State<Arc<Pool<MySql>>>,
) -> impl IntoResponse {
    let msg = give_type_accident(tyype, anio, stat, &state).await;
    let mut response = (StatusCode::OK, serde_json::to_string(&msg).unwrap()).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn page() -> impl IntoResponse {
    let html = std::fs::read_to_string("index.html").unwrap();

    let mut response = Html(html).into_response();
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"));
    response.headers_mut().append(
        ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("*"),
    );
    response
        .headers_mut()
        .append(ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("*"));
    response
}

async fn styles() -> impl IntoResponse {
    let css = std::fs::read_to_string("output.css").unwrap();

    let mut response = (StatusCode::OK, css).into_response();
    response.headers_mut().append(
        axum::http::header::CONTENT_TYPE,
        HeaderValue::from_static("text/css"),
    );

    response
}

#[tokio::main]
async fn main() {
    let state = Arc::new(
        MySqlPool::connect("mariadb://root:Pinky01*@127.0.0.1/accidents")
            .await
            .unwrap(),
    );

    let router = Router::new()
        .route("/styles.css", get(styles))
        .route("/query1/:anio", get(query1))
        .route("/query2/:anio", get(query2))
        .route("/query3/", get(query3))
        .route("/query4/", get(query4))
        .route("/query5/", get(query5))
        .route("/query6/", get(query6))
        .route("/query7/", get(query7))
        .route("/query8/", get(query8))
        .route("/query9/", get(query9))
        .route("/page", get(page))
        .with_state(state);

    Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
