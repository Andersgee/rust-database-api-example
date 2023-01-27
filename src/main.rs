use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod routes;

pub type Prisma = web::Data<db::PrismaClient>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("expected DATABASE_URL to exists in environment");
    let api_port = std::env::var("API_PORT").expect("expected API_PORT to exists in environment");

    let client = web::Data::new(
        db::new_client_with_url(&database_url)
            .await
            .expect("expected prisma client to successfully connect to database_url"),
    );

    #[cfg(debug)]
    client._push_db(false).await.unwrap();

    println!("listening on port {}", &api_port);
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(routes::users::get_all)
            .service(routes::users::get_by_id)
            .service(routes::examples::get_all)
            .service(routes::examples::get_by_id)
    })
    .bind(format!("0.0.0.0:{}", api_port))?
    .run()
    .await
}
