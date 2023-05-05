#![recursion_limit = "256"]
use leptos::*;

mod schema;
use diesel::*;
use schema::*;

#[derive(Queryable, PartialEq, Debug, Selectable)]
#[diesel(table_name = accidentes_2018)]
struct IdsAcc {
    id: i32,
    ID_ENTIDAD: String,
}

pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish("mysql://root:1234@localhost/Accidents").unwrap()
}

fn main() {
    use schema::accidentes_2018::dsl::*;

    let mut conection = establish_connection();
    let prueba = accidentes_2018
        .select((id, ID_ENTIDAD))
        .load::<IdsAcc>(&mut conection);
    println!("{:?}", prueba);
}
