use std::env;

use actix_cors::Cors;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use anyhow::Result;
use dotenv::dotenv;
use log::info;
use sqlx::SqlitePool;

mod shopping_list;

async fn index() -> impl Responder {
    HttpResponse::Ok().body(
        r#"
        Shopping list API

        TODO list of available endpoints...
        "#,
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").unwrap_or("[::]".to_owned());
    let port = env::var("PORT").unwrap_or("8080".to_owned());
    let address = format!("{}:{}", host, port);
    info!("Starting server on: http(s)://{}", address);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = SqlitePool::connect(&db_url).await?;
    info!("Using sqlite database at: {}", &db_url);

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            // add specific origin to allowed origin list
            // .allowed_origin("https://tanger46.duckdns.org")
            // .allowed_origin("http://localhost:3000")
            // allow any port on localhost
            // .allowed_origin_fn(|origin, _req_head| {
            //     origin.as_bytes().starts_with(b"http://localhost")
            // })
            .allow_any_origin()
            .send_wildcard()
            // .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allow_any_method()
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // .allowed_header(http::header::CONTENT_TYPE)
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(Data::new(db_pool.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .configure(shopping_list::config)
            .route("/", web::get().to(index))
    })
    .bind(address)?;
    server.run().await?;

    Ok(())
}
