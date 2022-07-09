#[macro_use]
extern crate diesel;

use actix_web::web::Data;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::io;
mod db;
mod model;
mod schema;

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[get("/helth-check")]
async fn helth_check() -> impl Responder {
    HttpResponse::Ok().body("Helth Check OK!")
}

#[get("/users/{id}")]
async fn get_user(
    db: web::Data<db::Pool>,
    path: web::Path<i32>,
) -> Result<impl Responder, io::Error> {
    let conn = db.get().unwrap();
    let id = path.into_inner();
    let user = schema::users::table
        .select(schema::users::email)
        .filter(schema::users::id.eq(id))
        .load::<String>(&conn)
        .expect("error");

    Ok(web::Json(user))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_user)
            .service(helth_check)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
