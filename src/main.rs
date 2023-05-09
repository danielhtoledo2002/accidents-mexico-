use anyhow::{anyhow, Error, Result};
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql};
use std::io::Write;
use std::time::Duration;
use tokio::time::error::Elapsed;

#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct AlcholStates {
    NOM_ENTIDAD: String,
    ALIENTO: String,
    borrachos: i32,
}

// struct to obtain the data from the table in mysql
#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct StateAccidentsSql {
    NOM_ENTIDAD: String,
    numero_accidentes: i32,
}
// Struct with real names
#[derive(Debug)]
struct StateAccidents {
    state: &'static str,
    accidentes: i32,
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
fn give_accidents_state(tabla: &str, pool: &sqlx::Pool<MySql>) -> Vec<StateAccidentsSql> {
    make_query::< StateAccidentsSql>(format!("select NOM_ENTIDAD, COUNT(accidentes_{0}.ID_ENTIDAD) as numero_accidentes from accidentes_{0},tc_entidad where tc_entidad.ID_ENTIDAD = accidentes_{0}.ID_ENTIDAD GROUP BY NOM_ENTIDAD HAVING COUNT(accidentes_{}.ID_ENTIDAD) > 1 ", tabla.trim()), 
     pool
    )    .unwrap()
}

//
// fn give_alcolics_state(tabla :&str, pool :&sqlx::Pool<MySql>) -> Vec<>

fn give_alcholic_state(tabla: &str, pool: &sqlx::Pool<MySql>) -> Vec<AlcholStates> {
    make_query::<AlcholStates>(format!("
          select  NOM_ENTIDAD, ALIENTO ,count(ALIENTO)as borrachos from accidentes_2018, tc_entidad where tc_entidad.ID_ENTIDAD = accidentes_{}.ID_ENTIDAD and accidentes_2018.ALIENTO = 'si' group by NOM_ENTIDAD, ALIENTO having  count(ALIENTO) >1 ", tabla.trim()), pool).unwrap()
}

// advertecia algunos id tienes que ponerlos como 09  como en el ID_ENTIDAD depende de comos e creo el csv (en mi caso en la laptop se creo sin el 0 y en la pc se creo 01)

fn main() {
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    let pool = rt.block_on(async {
        MySqlPool::connect("mysql://root:1234@localhost/Accidents")
            .await
            .unwrap()
    });
    // ya la muestra con el estado real y no con el id

    let mut estados_accidentes_2020 = give_accidents_state("2020", &pool);
    estados_accidentes_2020.sort_by(|a, b| a.numero_accidentes.cmp(&b.numero_accidentes)); //ordena de menor a mayor por el numero de accidentes por ciudad.
    println!("");
    println!("");
    println!("Acidentes por estado en el 2020");

    println!("{:?}", estados_accidentes_2020);

    let mut estados_accidentes_2019 = give_accidents_state("2019", &pool);

    estados_accidentes_2019.sort_by(|a, b| a.numero_accidentes.cmp(&b.numero_accidentes)); //ordena de menor a mayor por el numero de accidentes por ciudad.

    println!("");
    println!("");
    println!("Acidentes por estado en el 2019");
    println!("{:?}", estados_accidentes_2019);

    let mut estados_accidentes_2018 = give_accidents_state("2018", &pool);

    estados_accidentes_2018.sort_by(|a, b| a.numero_accidentes.cmp(&b.numero_accidentes));

    println!("");
    println!("");
    println!("Acidentes por estado en el 2018");
    println!("{:?}", estados_accidentes_2018);

    //query para saber que estados son los mas alcoholicos
    println!("");
    println!("");
    let mut alcholicos_estados_2018 = give_alcholic_state("2018", &pool);
    alcholicos_estados_2018.sort_by(|a, b| a.borrachos.cmp(&b.borrachos));

    println!("{:?}", alcholicos_estados_2018);
}
