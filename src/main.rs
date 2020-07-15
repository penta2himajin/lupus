mod lib;

use actix_web::{web, App, HttpServer};
use uuid::Uuid;
use crate::lib;

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let connection = establish_connection(
        "mysql://penta:kenya2318@localhost/lupus");

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