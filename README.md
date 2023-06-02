# Drashboard accidents México 2018-2021

## Database 
- The database we use was taken from [INEGI](https://www.inegi.org.mx/datosabiertos/) the only thing you need to do is to take the csv from the years and put it into unique file.
![image](https://github.com/danielhtoledo2002/accidents-mexico-/assets/92064764/6356bc2c-4e54-4995-a789-7c4c9bc089d9)
- you need to transform the data from csv file to mariadb data. 
  - you can use what ever you want, I recomended datagrip.


## Structure 
We use Rust to run the web server and control the backend using axum and sqlx to do  queries.
This funtion help us to do more quickly queries.
```Rust
pub async fn make_query<T>(query: impl AsRef<str>, connection: &sqlx::Pool<MySql>) -> Result<Vec<T>>
    where for<'a> T: sqlx::FromRow<'a, sqlx::mysql::MySqlRow> + Send + Unpin
{
    let result: Vec<T> = sqlx::query_as::< _,T> (query.as_ref())
        .fetch_all(connection)
        .await ?;

    if result.is_empty() {
        Err(anyhow!("No se encontró ningún dato"))
    } else {
        Ok(result)
    }
}
```
- We use html, javascript and tailwind to build the frontend. 
-  Javascript
  - Alpinejs 
  - Chartjs 
- We use Json protocol to comunicate the frontend to backend.

## How to run 
If you have rust installed  the only thing you need to do is `cargo run`.
If you don't  install rust using `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` then `cargo run`.

