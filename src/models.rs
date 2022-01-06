use super::errors::Result;
use super::schema::users;
use super::schema::users::dsl::users as all_users;
use super::Pool;
use diesel::{Insertable, QueryDsl, RunQueryDsl};
use serde::Serialize;
use std::time::SystemTime;

pub struct UserId(pub i32);

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub first_name: &'a str,
    pub created_at: SystemTime,
}

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub created_at: SystemTime,
}

impl User {
    pub fn get_all_users(db: &Pool) -> Result<Vec<User>> {
        let conn = db.get()?;
        Ok(all_users.load::<User>(&conn)?)
    }

    pub fn insert_user(user: NewUser, db: &Pool) -> Result<usize> {
        let conn = db.get()?;
        Ok(diesel::insert_into(users::table)
            .values(&user)
            .execute(&conn)?)
    }

    pub fn get_user_by_id(userid: UserId, db: &Pool) -> Result<User> {
        let conn = db.get()?;
        Ok(all_users.find(userid.0).get_result::<User>(&conn)?)
    }

    pub fn delete_user_by_id(userid: UserId, db: &Pool) -> Result<usize> {
        let conn = db.get()?;

        let count = diesel::delete(all_users.find(userid.0)).execute(&conn)?;
        Ok(count)
    }
}
