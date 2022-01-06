//! This crate provides a cross-platform library and binary for translating addresses into

#[macro_use]
extern crate diesel;

use actix_web::{middleware::Logger, web, App, HttpServer};
use clap::Parser;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use errors::{CustomError, Result};
use std::net;

mod errors;
mod handlers;
mod models;
mod schema;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Specify alternate bind address [default: 127.0.0.1:8000]
    #[clap(short, long)]
    bind: Option<String>,

    /// Log to stderr
    #[clap(short, long)]
    debug: bool,
}

async fn run<A, S>(database_url: S, bind: A) -> Result<()>
where
    A: net::ToSocketAddrs,
    S: Into<String>,
{
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .map_err(|_| CustomError::Message("Failed to create pool.".into()))?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user_by_id))
    })
    .bind(bind)
    .map_err(|_| CustomError::Message("Failed to bind".into()))?
    .run()
    .await
    .map_err(|_| CustomError::Message("Failed to run webserver".into()))?;

    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let args = Args::parse();

    if args.debug {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }

    let bind_address = match args.bind.as_ref() {
        Some(s) => s,
        None => "127.0.0.1:8000",
    };

    env_logger::init();

    run(database_url, bind_address).await?;

    Ok(())
}
