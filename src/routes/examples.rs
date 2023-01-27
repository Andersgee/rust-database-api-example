use crate::db::example;
use actix_web::{get, web, HttpResponse, Responder};

use crate::Prisma;

#[get("/examples")]
pub async fn get_all(prisma: Prisma) -> impl Responder {
    let examples = prisma.example().find_many(vec![]).exec().await.unwrap();

    HttpResponse::Ok().json(examples)
}

#[get("/examples/{id}")]
pub async fn get_by_id(prisma: Prisma, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let example: Option<example::Data> = prisma
        .example()
        .find_unique(example::id::equals(id))
        .exec()
        .await
        .unwrap();

    match example {
        Some(data) => HttpResponse::Ok().json(data),
        None => HttpResponse::NotFound().json("NotFound"),
    }
}
