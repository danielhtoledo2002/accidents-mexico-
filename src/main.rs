use anyhow::{anyhow, Result};
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql, Pool};

use std::process::ExitCode;
use std::sync::Arc;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado6 {
    conduc: Vec<Decimal>,
    pasajero: Vec<Decimal>,
    peaton: Vec<Decimal>,
    ciclista: Vec<Decimal>,
    otro: Vec<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado7 {
    sexo: Vec<String>,
    conduc: Vec<Decimal>,
    pasajero: Vec<Decimal>,
    peaton: Vec<Decimal>,
    ciclista: Vec<Decimal>,
    otro: Vec<Decimal>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado8 {
    anio: Vec<i32>,
    conduc: Vec<Decimal>,
    pasajero: Vec<Decimal>,
    peaton: Vec<Decimal>,
    ciclista: Vec<Decimal>,
    otro: Vec<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resultado9 {
    AUTOMOVIL: Vec<Decimal>,
    CAMPASAJ: Vec<Decimal>,
    MICROBUS: Vec<Decimal>,
    PASCAMION: Vec<Decimal>,
    OMNIBUS: Vec<Decimal>,
    TRANVIA: Vec<Decimal>,
    CAMIONETA: Vec<Decimal>,
    CAMION: Vec<Decimal>,
    TRACTOR: Vec<Decimal>,
    FERROCARRI: Vec<Decimal>,
    MOTOCICLET: Vec<Decimal>,
    BICICLETA: Vec<Decimal>,
    OTROVEHIC: Vec<Decimal>,
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

    let yo: Vec<(String, Decimal)> = make_query::<(String, Decimal)>("SELECT 'Zona Rural' as rural ,SUM(total) as accidentes_rurales
        from(select count(SUBURBANA) as total from atus_anual where SUBURBANA != 'Sin accidente en esta zona'  group by SUBURBANA) a", pool).await.unwrap();

    y.extend(yo);

    Resultado3 {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// give the day and hour from accidedents

async fn give_day_hour_accidents(pool: &sqlx::Pool<MySql>) -> Resultado4 {
    let y: Vec<(String, i32, i32)> = make_query::<(String, i32, i32)>(
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
    let y: Vec<(i32, i32)> = make_query::<(i32, i32)>(
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
    let y: Vec<(String, i32)> = make_query::<(String, i32)>(
        "select tipaccid, count(id)as total from atus_anual group by tipaccid order by total desc ;
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
    let y: Vec<(String, i32)> = make_query::<(String, i32)>(
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
    let y: Vec<(String, i32)> = make_query::<(String, i32)>(
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

// age / total accidens
async fn give_age_accidents(pool: &sqlx::Pool<MySql>) -> Resultado2 {
    let y : Vec<(i32, i32)> = make_query::<(i32, i32)>("select id_edad, count(id_edad) as total from atus_anual group by id_edad order by total desc;", pool).await.unwrap();
    Resultado2 {
        x: y.iter().map(|a| a.0).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// /fatal/solo daños/ no fatal/solo daños /certificacion 0 por año (agregar boton para cambar de año)
async fn give_total_result_accident(anio: i32, pool: &sqlx::Pool<MySql>) -> Resultado6 {
    let y : Vec<(Decimal, Decimal, Decimal, Decimal, Decimal)> = make_query::<(Decimal, Decimal, Decimal, Decimal, Decimal)>(format!("select  sum(condmuerto), sum(pasamuerto), sum(peatmuerto), sum(ciclmuerto), sum(otromuerto)
        from atus_anual where anio = {};", anio), pool).await.unwrap();
    Resultado6 {
        conduc: y.iter().map(|a| a.0).collect(),
        pasajero: y.iter().map(|a| a.1).collect(),
        peaton: y.iter().map(|a| a.2).collect(),
        ciclista: y.iter().map(|a| a.3).collect(),
        otro: y.iter().map(|a| a.4).collect(),
    }
}

// te da el numero de muertos por cada accidente o tipo
async fn give_general_deads(pool: &sqlx::Pool<MySql>) -> Resultado8 {
    let y : Vec<(i32, Decimal, Decimal, Decimal, Decimal, Decimal)> = make_query::<(i32,Decimal, Decimal, Decimal, Decimal, Decimal)>("select ANIO,  sum(condmuerto), sum(pasamuerto), sum(peatmuerto), sum(ciclmuerto), sum(otromuerto)
        from atus_anual  group by ANIO;", pool).await.unwrap();
    Resultado8 {
        anio: y.iter().map(|a| a.0).collect(),
        conduc: y.iter().map(|a| a.1).collect(),
        pasajero: y.iter().map(|a| a.2).collect(),
        peaton: y.iter().map(|a| a.3).collect(),
        ciclista: y.iter().map(|a| a.4).collect(),
        otro: y.iter().map(|a| a.5).collect(),
    }
}

// te da la muertes por sexo en accidente auto/ pasajero etc
async fn give_general_deads_sexo(pool: &sqlx::Pool<MySql>) -> Resultado7 {
    let y : Vec<(String, Decimal, Decimal, Decimal, Decimal, Decimal)> = make_query::<(String,Decimal, Decimal, Decimal, Decimal, Decimal)>("select SEXO,  sum(condmuerto), sum(pasamuerto), sum(peatmuerto), sum(ciclmuerto), sum(otromuerto)
        from atus_anual WHERE SEXO != 'Certificado cero' group by SEXO;", pool).await.unwrap();
    Resultado7 {
        sexo: y.iter().map(|a| a.0.clone()).collect(),
        conduc: y.iter().map(|a| a.1).collect(),
        pasajero: y.iter().map(|a| a.2).collect(),
        peaton: y.iter().map(|a| a.3).collect(),
        ciclista: y.iter().map(|a| a.4).collect(),
        otro: y.iter().map(|a| a.5).collect(),
    }
}
// borrachos vs no borrachos se ignora y hay otro
async fn give_aliento_noaliento(pool: &sqlx::Pool<MySql>) -> Resultado {
    let y: Vec<(String, i32)> = make_query::<(String, i32)>(
        "select aliento, count(aliento) from atus_anual group by aliento;",
        pool,
    )
    .await
    .unwrap();
    Resultado {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

// query de vehiculos chocados
async fn give_sum_vehicles(pool: &sqlx::Pool<MySql>) -> Resultado9 {
    let y: Vec<(
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
    )> = make_query::<(
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
        Decimal,
    )>(
        "Select sum(AUTOMOVIL) as auto, sum(CAMPASAJ) as camion,
sum(MICROBUS) as microbus, sum(OMNIBUS)as omnibus,
sum(PASCAMION) as camion_pasajeros, sum(TRANVIA) as tranvia,
sum(CAMIONETA) as camioneta, sum(CAMION)as camion,
sum(TRACTOR) as tractor, sum(FERROCARRI) as ferrocarril,
sum(MOTOCICLET) as moto,
sum(BICICLETA) as bici,
sum(OTROVEHIC) as otro From  atus_anual;",
        pool,
    )
    .await
    .unwrap();
    Resultado9 {
        AUTOMOVIL: y.iter().map(|a| a.0).collect(),
        CAMPASAJ: y.iter().map(|a| a.1).collect(),
        MICROBUS: y.iter().map(|a| a.2).collect(),
        PASCAMION: y.iter().map(|a| a.3).collect(),
        OMNIBUS: y.iter().map(|a| a.4).collect(),
        TRANVIA: y.iter().map(|a| a.5).collect(),
        CAMIONETA: y.iter().map(|a| a.6).collect(),
        CAMION: y.iter().map(|a| a.7).collect(),
        TRACTOR: y.iter().map(|a| a.8).collect(),
        FERROCARRI: y.iter().map(|a| a.9).collect(),
        MOTOCICLET: y.iter().map(|a| a.10).collect(),
        BICICLETA: y.iter().map(|a| a.11).collect(),
        OTROVEHIC: y.iter().map(|a| a.12).collect(),
    }
}

// causante del accidente

async fn give_cause_accident(pool: &sqlx::Pool<MySql>) -> Resultado {
    let y: Vec<(String, i32)> = make_query::<(String, i32)>(
        "select causaacci, count(causaacci) from atus_anual group by causaacci;",
        pool,
    )
    .await
    .unwrap();
    Resultado {
        x: y.iter().map(|a| a.0.clone()).collect(),
        y: y.iter().map(|a| a.1).collect(),
    }
}

/*


*/

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
    Path((tyype, anio, stat)): Path<(&str, i32, &str)>,
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
async fn query11(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_age_accidents(&state).await;
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

async fn query12(
    Path(anio): Path<i32>,
    State(state): State<Arc<Pool<MySql>>>,
) -> impl IntoResponse {
    let msg = give_total_result_accident(anio, &state).await;

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
async fn query13(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_general_deads(&state).await;
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
async fn query14(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_general_deads_sexo(&state).await;
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
async fn query15(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_aliento_noaliento(&state).await;
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

async fn query16(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_sum_vehicles(&state).await;
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

async fn query17(State(state): State<Arc<Pool<MySql>>>) -> impl IntoResponse {
    let msg = give_cause_accident(&state).await;
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
async fn main() -> ExitCode {
    if let Ok(maria_address) = std::env::var("MARIA_SERVER_ADDRESS") {
        let state = Arc::new(MySqlPool::connect(&maria_address).await.unwrap());

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
            .route("/query11/", get(query11))
            .route("/query12/:anio", get(query12))
            .route("/query13/", get(query13))
            .route("/query14/", get(query14))
            .route("/query15/", get(query15))
            .route("/query16/", get(query16))
            .route("/query17/", get(query17))
            .route("/page", get(page))
            .with_state(state);

        Server::bind(&"0.0.0.0:80".parse().unwrap())
            .serve(router.into_make_service())
            .await
            .unwrap();

        ExitCode::SUCCESS
    } else {
        println!("MARIA_SERVER_ADDRESS is not set");
        ExitCode::FAILURE
    }
}
