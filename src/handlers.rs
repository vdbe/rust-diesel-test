use super::models::{NewUser, User, UserId};
use super::Pool;
use actix_web::{web, Error, HttpResponse};
use serde::Deserialize;
use std::time::SystemTime;

#[derive(Debug, Deserialize)]
pub struct InputUser {
    pub first_name: String,
    pub username: String,
    pub password: String,
}

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(User::get_all_users(db.get_ref())
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn get_user_by_id(
    db: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = UserId(id.0);

    Ok(User::get_user_by_id(user_id, db.get_ref())
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn add_user(
    db: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    let new_user = NewUser {
        username: &item.username,
        password: &item.password,
        first_name: &item.first_name,
        created_at: SystemTime::now(),
    };

    Ok(User::insert_user(new_user, db.get_ref())
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

pub async fn delete_user_by_id(
    db: web::Data<Pool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let user_id = UserId(id.0);

    Ok(User::delete_user_by_id(user_id, db.get_ref())
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}
