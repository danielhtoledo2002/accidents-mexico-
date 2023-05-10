use anyhow::{anyhow, Error, Result};
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql};
use std::io::Write;
use std::time::Duration;
use tokio::time::error::Elapsed;

#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct AlcholStates {
    NOM_ENTIDAD: String,
    borrachos: i32,
}

// struct to obtain the data from the table in mysql
#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct StateAccidentsSql {
    NOM_ENTIDAD: String,
    numero_accidentes: i32,
}

//struct to obtain number for each type of accident by state

#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct TypeAccidentByState {
    NOM_ENTIDAD: String,
    tipo: String,
    cantidad: i32,
}

// input method
pub fn input(msg: &str) -> Result<String> {
    let mut h = std::io::stdout();
    write!(h, "{}", msg).unwrap();
    h.flush().unwrap();

    let mut campos = String::new();
    let _ = std::io::stdin().read_line(&mut campos)?;
    Ok(campos.trim().to_string())
}

// function to fill a generic struct

fn make_query<T>(query: impl AsRef<str>, connection: &sqlx::Pool<MySql>) -> Result<Vec<T>>
where
    for<'a> T: sqlx::FromRow<'a, sqlx::mysql::MySqlRow> + Send + Unpin,
{
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let result: Vec<T> = sqlx::query_as::<_, T>(query.as_ref())
            .fetch_all(connection)
            .await?;

        if result.is_empty() {
            Err(anyhow!("No se encontró ningún dato"))
        } else {
            Ok(result)
        }
    })
}

// Give me all the accidents that happen in a state in the year without any filter
fn give_accidents_state(anio: i32, pool: &sqlx::Pool<MySql>) -> Vec<StateAccidentsSql> {
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
    )
    .unwrap()
}

// fn give_alcolics_state(tabla :&str, pool :&sqlx::Pool<MySql>) -> Vec<>

fn give_alcholic_state(anio: i32, pool: &sqlx::Pool<MySql>) -> Vec<AlcholStates> {
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
    )
    .unwrap()
}

fn give_type_accident(tyype: &str, pool: &sqlx::Pool<MySql>, anio : i32, state : i32) -> Vec<TypeAccidentByState>{
    make_query::<TypeAccidentByState>(format!("
    select NOM_ENTIDAD, tipo, cantidad from (select ID_ENTIDAD, COUNT(ID_ENTIDAD) AS
    cantidad,TIPACCID as tipo  from atus_anual where ANIO = {1} AND TIPACCID = '{0}' and ID_ENTIDAD = {2} group by ID_ENTIDAD, TIPACCID) AS res
    join tc_entidad using(ID_ENTIDAD);
    ", tyype.trim(), anio, state) ,pool).unwrap()

}


// advertecia algunos id tienes que ponerlos como 09  como en el ID_ENTIDAD depende de comos e creo el csv (en mi caso en la laptop se creo sin el 0 y en la pc se creo 01)

fn main() {
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    let pool = rt.block_on(async {
        MySqlPool::connect("mysql://root:aesr@187.208.119.8/accidents")
            .await
            .unwrap()
    });
    // ya la muestra con el estado real y no con el id

    let mut estados_accidentes_2020 = give_accidents_state(2020, &pool);
    estados_accidentes_2020.sort_by(|a, b| a.numero_accidentes.cmp(&b.numero_accidentes)); //ordena de menor a mayor por el numero de accidentes por ciudad.
    println!("");
    println!("");
    println!("Acidentes por estado en el 2020");

    println!("{:?}", estados_accidentes_2020);

    let mut estados_accidentes_2019 = give_accidents_state(2019, &pool);

    estados_accidentes_2019.sort_by(|a, b| a.numero_accidentes.cmp(&b.numero_accidentes)); //ordena de menor a mayor por el numero de accidentes por ciudad.

    println!("");
    println!("");
    println!("Acidentes por estado en el 2019");
    println!("{:?}", estados_accidentes_2019);

    let mut estados_accidentes_2018 = give_accidents_state(2018, &pool);

    estados_accidentes_2018.sort_by(|a, b| a.numero_accidentes.cmp(&b.numero_accidentes));

    println!("");
    println!("");
    println!("Acidentes por estado en el 2018");
    println!("{:?}", estados_accidentes_2018);

    //query para saber que estados son los mas alcoholicos
    println!("");
    println!("");
    println!("Número de accidentes por sustancias alcoholicas");
    let mut alcholicos_estados_2018 = give_alcholic_state(2018, &pool);
    alcholicos_estados_2018.sort_by(|a, b| a.borrachos.cmp(&b.borrachos));

    println!("{:?}", alcholicos_estados_2018);


    println!("");
    println!("");
    println!("Número de tipos de accidente por estado ");

    let mut type_accident = give_type_accident("Colisión con ciclista", &pool,2018, 9 );


    type_accident.sort_by(|a, b| a.cantidad.cmp(&b.cantidad));

    println!("{:?}", type_accident);


}
