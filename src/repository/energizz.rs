use crate::entity::{prelude::*, energizz};
use actix_web::web::Json;
use log::debug;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveValue::NotSet};
use sea_orm::{entity::*, query::*, DeriveEntityModel, DeleteResult};
use serde::{Serialize, Deserialize};
use sea_orm::prelude::Date as SeaOrmDate;
use sea_orm::entity::prelude::*;

use sea_orm::{EnumIter, Iterable};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
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


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnergizzRequest {
    pub id: i32,
    pub temperatura: f32,
    pub umidade: f32,
    pub pressao: f32,
    pub paciente_id: String,
    pub posting_time: String,
}

#[derive(Debug, Clone)]
pub struct EnergizzRepository {
    pub db_conn: DatabaseConnection,
}

impl EnergizzRepository {
    pub async fn get_energizz(&self) -> Vec<energizz::Model> {
        Energizz::find()
            .all(&self.db_conn)
            .await
            .expect("Failed to get energizz")
    }

    pub async fn get_energizz_by_id(&self, id: i32) -> Option<energizz::Model> {
        Energizz::find_by_id(id)
            .one(&self.db_conn)
            .await
            .expect("Failed to get energizz")
    }

    pub async fn create_energizz(&self, new_energizz: Json<EnergizzRequest>) -> Option<energizz::Model> {
        let energizz = energizz::ActiveModel {
            id: NotSet,
            temperatura: ActiveValue::Set(new_energizz.temperatura.to_owned()),
            umidade: ActiveValue::Set(new_energizz.umidade.to_owned()),
            pressao: ActiveValue::Set(new_energizz.pressao.to_owned()),
            paciente_id: ActiveValue::Set(new_energizz.paciente_id.to_owned()),
           // posting_time: ActiveValue::Set(new_energizz.posting_time), // Use diretamente o campo posting_time
            posting_time: ActiveValue::Set(new_energizz.posting_time.to_owned()),
        };

        let energizz: energizz::Model = energizz.insert(&self.db_conn).await.unwrap();
        debug!("Created energizz: energizz{}", energizz.temperatura);
        return energizz.into();
    }

    pub async fn update_energizz(
        &self,
        id: i32,
        new_energizz: Json<EnergizzRequest>,
    ) -> Option<energizz::Model> {
        let energizz = Energizz::find_by_id(id)
            .one(&self.db_conn)
            .await
            .expect("Failed to get energizz");
        let mut energizz: energizz::ActiveModel = energizz.unwrap().into();
        energizz.temperatura = ActiveValue::Set(new_energizz.temperatura.to_owned());
        energizz.umidade = ActiveValue::Set(new_energizz.umidade.to_owned());
        energizz.pressao = ActiveValue::Set(new_energizz.pressao.to_owned());
        energizz.paciente_id = ActiveValue::Set(new_energizz.paciente_id.to_owned());
       // energizz.posting_time = ActiveValue::Set(new_energizz.posting_time); // Use diretamente o campo posting_time
        energizz.posting_time = ActiveValue::Set(new_energizz.posting_time.to_owned());

        let energizz: energizz::Model = energizz.update(&self.db_conn).await.unwrap();
        debug!("Updated energizz: energizz{}", energizz.temperatura);
        return energizz.into();
    }

    pub async fn delete_energizz_by_id(&self, id: i32) -> DeleteResult {
        let res: DeleteResult = Energizz::delete_by_id(id).exec(&self.db_conn).await.unwrap();

        return res.into();
    }
        
}