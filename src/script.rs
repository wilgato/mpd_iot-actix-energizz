use sea_orm_migration::prelude::*;

mod entity;
pub use entity::*;

#[async_std::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = sea_orm::Database::connect(&db_url).await.unwrap();

    run_migrations(&db_conn).await.expect("Failed to run migrations");
}

// Função auxiliar para executar todas as migrações
pub async fn run_migrations(db_conn: &sea_orm::DatabaseConnection) -> Result<(), DbErr> {
    Migrator::migrate_all(db_conn, &Migrator).await?;
    Ok(())
}
