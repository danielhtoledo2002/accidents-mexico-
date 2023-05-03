use leptos::*;
mod schema;
use diesel::*;
use schema::*;

#[derive(Queryable, PartialEq, Debug, Selectable)]
#[diesel(table_name = cards)]
struct Cards {
    number: String,
    bank_id: u32,
}

pub fn establish_connection() -> MysqlConnection {
    MysqlConnection::establish("mysql://daniel:1234@localhost/banco").unwrap()
}

fn main() {
    use schema::cards::dsl::*;
    let mut conection = establish_connection();
    let x = cards
        .select((number, bank_id))
        .load::<Cards>(&mut conection);
    println!("{:?}", x);
}
