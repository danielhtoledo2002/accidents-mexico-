use anyhow::{anyhow, Error, Result};
use sqlx::mysql::MySqlPool;
use sqlx::{Connection, MySql};
use std::time::Duration;
use tokio::time::error::Elapsed;

#[derive(Debug, sqlx::FromRow, Clone, PartialEq, PartialOrd, Default)]
struct StateAccidentsSql {
    ID_ENTIDAD: String,
    numero_accidentes: i32,
}

#[derive(Debug)]
struct StateAccidents{
    state: &'static str,
    accidentes: i32,
}

fn construct(estado: &str, acci: i32)->StateAccidents{
    match estado.trim() {
        "01" => StateAccidents { state: "Aguascalientes", accidentes: acci },
        "02" => StateAccidents { state: "Baja California", accidentes: acci },
        "03" => StateAccidents { state: "Baja California Sur", accidentes: acci  },
        "04" => StateAccidents { state: "Campeche", accidentes: acci  },
        "05" => StateAccidents { state: "Coahuila", accidentes:acci  },
        "06" =>StateAccidents { state: "Colima", accidentes: acci },
        "07" => StateAccidents { state: "Chiapas", accidentes:acci  },
        "08" => StateAccidents { state: "Chihuhua", accidentes:acci  },
        "09" => StateAccidents { state: "Ciudad de Mexico", accidentes:acci  },
        "10" => StateAccidents { state: "Durango", accidentes: acci },
        "11" => StateAccidents { state: "Guanajuato", accidentes:acci  },
        "12"=>StateAccidents { state: "Guerrero", accidentes: acci},
        "13"=>StateAccidents { state: "Hidalgo", accidentes: acci},
        "14"=>StateAccidents { state: "Jalisco", accidentes: acci},
        "15"=>StateAccidents { state: "Mexico", accidentes: acci},
        "16"=>StateAccidents { state: "Michoacan ", accidentes: acci},
        "17"=>StateAccidents { state: "Morelos", accidentes: acci},
        "18"=>StateAccidents { state: "Nayarit", accidentes: acci},
        "19"=>StateAccidents { state: "Nuevo Leon", accidentes: acci},
        "20"=>StateAccidents { state: "Oaxaca", accidentes: acci},
        "21"=>StateAccidents { state: "Puebla", accidentes: acci},
        "22"=>StateAccidents { state: "Queretaro", accidentes: acci},
        "23"=>StateAccidents { state: "Quintana Roo", accidentes: acci},
        "24"=>StateAccidents { state: "San Luis Potosi", accidentes: acci},
        "25"=>StateAccidents { state: "Sinaloa", accidentes: acci},
        "26"=>StateAccidents { state: "Sonora", accidentes: acci},
        "27"=>StateAccidents { state: "Tabasco", accidentes: acci},
        "28"=>StateAccidents { state: "Tamaulipas", accidentes: acci},
        "29"=>StateAccidents { state: "Tlaxcala", accidentes: acci},
        "30"=>StateAccidents { state: "Veracruz", accidentes: acci},
        "31"=>StateAccidents { state: "Yucatan", accidentes: acci},
        "32"=>StateAccidents { state: "Zacatecas", accidentes: acci},

        _ => unreachable!("Estado no identificado")
        
        
     }
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
    // ya la muestra con el estado real y no con el id

    println!("");
    println!("");
    println!("Acidentes por estado en el 2020"); 
    let mut estados_accidentes_2020:Vec<_> = make_query::<StateAccidentsSql>("select ID_ENTIDAD, COUNT(ID_ENTIDAD) as numero_accidentes from accidentes_2020 GROUP BY ID_ENTIDAD HAVING COUNT(ID_ENTIDAD) > 1 ",
        &mut pool
    )
    .await    
    .unwrap().into_iter().map(|i| construct(i.ID_ENTIDAD.as_str(),i.numero_accidentes )).collect();

    estados_accidentes_2020.sort_by(|a, b| a.accidentes.cmp(&b.accidentes)); //ordena de menor a mayor por el numero de accidentes por ciudad.
       
    println!("{:?}", estados_accidentes_2020);
    println!("");
    println!("");
    println!("Accidentes por estado en el 2021");
   
    let mut estados_accidentes_2021:Vec<_> = make_query::<StateAccidentsSql>("select ID_ENTIDAD, COUNT(ID_ENTIDAD) as numero_accidentes from accidentes_2021 GROUP BY ID_ENTIDAD HAVING COUNT(ID_ENTIDAD) > 1 ",
        &mut pool
    )
    .await    
    .unwrap().into_iter().map(|a| construct(a.ID_ENTIDAD.as_str(),a.numero_accidentes )).collect();

    estados_accidentes_2021.sort_by(|c, d| c.accidentes.cmp(&d.accidentes));  
       
    println!("{:?}", estados_accidentes_2021);


}
