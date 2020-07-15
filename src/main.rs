#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use uuid::Uuid;

mod route;
mod models;
mod schema;
mod sql;

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let conn = sql::establish_connection();
    /* sql::create_user(
        &conn,
        "550e8400-e29b-41d4-a716-446655440000",
        "penta2himajin",
        &false,
    ).unwrap(); */
    let users = sql::show_users(&conn, None).unwrap();
    for user in users {
        println!("{}", user.username);
    }

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(route::index))
            .route("/again", web::get().to(route::index2))
            .route("/cat", web::get().to(route::cat))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}