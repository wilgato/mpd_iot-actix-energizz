use crate::repository::prelude::EnergizzRepository;
use handler::prelude::energizz_handler;
mod entity;
mod handler;
mod repository;
use actix_files as fs;
use actix_web::{web::{self, Data}, App, HttpServer, middleware};
use sea_orm::DatabaseConnection;
use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    pub energizz_repository: EnergizzRepository,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    dotenv::dotenv().ok();
    env_logger::init();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = sea_orm::Database::connect(&db_url).await.unwrap();
    
    let host = std::env::var("HOST").expect("HOST must be set");
    let port = std::env::var("PORT").expect("PORT must be set");
    let server_url = format!("{}:{}", host, port);

    let energizz_repository = EnergizzRepository {
        db_conn: db_conn.clone(),
    };

    let state: AppState = AppState {
        energizz_repository: energizz_repository,
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(middleware::Logger::default())
            .configure(init)
            .service(fs::Files::new("/static", "static"))
            //.service(actix_files::Files::new("/migrations", "migrations"))
    })
    .bind(&server_url)?;

    println!("Server running on {}", server_url);

    server.run().await?;

    Ok(())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(energizz_handler());

    // Serve a página inicial
    cfg.route("/", web::get().to(index));

    // Serve the dashboard page
    cfg.route("/dashboard", web::get().to(dashboard));
}

async fn index() -> Result<fs::NamedFile, actix_web::Error> {
    // Caminho relativo para a página inicial
    let path = "static/inicio.html";

    // Tente carregar o arquivo e tratá-lo como um arquivo estático
    let named_file = fs::NamedFile::open(path)?;

    Ok(named_file)
}

async fn dashboard() -> Result<fs::NamedFile, actix_web::Error> {
    let path = "static/dashboard.html";
    let named_file = fs::NamedFile::open(path)?;
    Ok(named_file)
}