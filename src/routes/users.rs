use crate::db::user;
use actix_web::{get, web, HttpResponse, Responder};

use crate::Prisma;

#[get("/users")]
pub async fn get_all(prisma: Prisma) -> impl Responder {
    let examples = prisma.user().find_many(vec![]).exec().await.unwrap();

    HttpResponse::Ok().json(examples)
}

#[get("/users/{id}")]
pub async fn get_by_id(prisma: Prisma, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    let user: Option<user::Data> = prisma
        .user()
        .find_unique(user::id::equals(id))
        .exec()
        .await
        .unwrap();

    match user {
        Some(data) => HttpResponse::Ok().json(data),
        None => HttpResponse::NotFound().json("NotFound"),
    }
}
