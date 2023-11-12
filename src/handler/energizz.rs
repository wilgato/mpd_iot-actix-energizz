use crate::entity::energizz::Model as Energizz;
use crate::AppState;
use actix_files as fs;
use actix_web::{
    get, post, web, web::Json, Error as ActixError, Responder, Result as ActixResult, Scope, put, delete,
};
use log::{debug};
use sea_orm::DeleteResult;
use serde_json::json;

#[get("")]
async fn get_energizz(state: web::Data<AppState>) -> ActixResult<impl Responder, ActixError> {
    let energizz = state.energizz_repository.get_energizz().await;
    Ok(web::Json(energizz))
}

#[get("/{id}")]
async fn get_energizz_by_id(state: web::Data<AppState>, id: web::Path<i32>) -> ActixResult<impl Responder, ActixError> {
    let energizz = state.energizz_repository.get_energizz_by_id(id.into_inner()).await;
    Ok(web::Json(energizz))
}

#[post("")]
async fn create_energizz(state: web::Data<AppState>, new_energizz: Json<crate::repository::energizz::EnergizzRequest>) -> ActixResult<impl Responder, ActixError> {
    let energizz = state.energizz_repository.create_energizz(new_energizz).await;
    Ok(web::Json(energizz))
}

#[put("/{id}")]
async fn update_energizz(
    state: web::Data<AppState>,
    id: web::Path<i32>,
    updated_energizz: web::Json<crate::repository::energizz::EnergizzRequest>,
) -> ActixResult<impl Responder, ActixError> {
    let energizz = state
        .energizz_repository
        .update_energizz(id.into_inner(), updated_energizz)
        .await;
    Ok(web::Json(energizz))
}

#[delete("/{id}")]
async fn delete_energizz(
    state: web::Data<AppState>,
    id: web::Path<i32>,
) -> ActixResult<impl Responder, ActixError> {
    let energizz: sea_orm::DeleteResult =
        state.energizz_repository.delete_energizz_by_id(id.into_inner()).await;
    Ok(web::Json(json!({
        "message": "Todo deleted successfully",
        "deleted": energizz.rows_affected
    })))
}

#[get("/dashboard")]
async fn dashboard() -> Result<actix_files::NamedFile, actix_web::Error> {
    let path = "static/dashboard.html";
    let named_file = actix_files::NamedFile::open(path)?;
    Ok(named_file)
}


pub fn energizz_handler() -> Scope {
    web::scope("/energizz")
        .service(get_energizz)
        .service(get_energizz_by_id)
        .service(create_energizz)
        .service(update_energizz)
        .service(delete_energizz)
        .service(dashboard)
}