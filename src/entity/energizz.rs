//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "energizz")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Float")]
    pub temperatura: f32,
    #[sea_orm(column_type = "Float")]
    pub umidade: f32,
    #[sea_orm(column_type = "Float")]
    pub pressao: f32,
    #[sea_orm(column_type = "Text")]
    pub paciente_id: String,
    #[sea_orm(column_type = "Text")]
    pub posting_time: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
