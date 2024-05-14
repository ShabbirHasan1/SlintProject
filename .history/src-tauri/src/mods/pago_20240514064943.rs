<<<<<<< HEAD
use entity::prelude::MedioDB;
use rand::random;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
=======
use super::lib::Save;
use entity::prelude::{MedioDB, PagoDB};
use rand::random;
use sea_orm::{ActiveModelTrait, ColumnTrait, Database, DbErr, EntityTrait, QueryFilter, Set};
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::async_runtime;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedioPago {
    medio: Arc<str>,
    id: i32,
}

impl MedioPago {
    pub fn new(medio: &str, id: i32) -> MedioPago {
        MedioPago {
            medio: Arc::from(medio),
            id,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
    pub fn desc(&self) -> Arc<str> {
        Arc::clone(&self.medio)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pago {
    int_id: u32,
    medio_pago: MedioPago,
    monto: f32,
    pagado: f32,
}

impl Pago {
    pub fn new(medio_pago: MedioPago, monto: f32, pagado: Option<f32>) -> Pago {
        let int_id = random();

        Pago {
            medio_pago,
            monto,
            int_id,
            pagado: match pagado {
                Some(a) => a,
                None => monto,
            },
        }
    }
    pub fn medio_pago(&self) -> &MedioPago {
        &self.medio_pago
    }
    pub fn medio(&self) -> Arc<str> {
        Arc::clone(&self.medio_pago.medio)
    }
    pub fn monto(&self) -> f32 {
        self.monto
    }
    pub fn id(&self) -> u32 {
        self.int_id
    }
    pub fn pagado(&self) -> &f32 {
        &self.pagado
    }
<<<<<<< HEAD
    pub fn def(db: &DatabaseConnection) -> Self {
        let res = async_runtime::block_on(medio_from_db("Efectivo", db));
=======
}
impl Save for Pago {
    async fn save(&self) -> Result<(), DbErr> {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc").await?;
        let medio_id = MedioDB::Entity::find()
            .filter(MedioDB::Column::Medio.eq(self.medio().to_string()))
            .one(&db)
            .await?
            .unwrap();
        let model = PagoDB::ActiveModel {
            medio_pago: Set(medio_id.id),
            monto: Set(self.monto),
            ..Default::default()
        };
        model.insert(&db).await?;
        Ok(())
    }
}

pub async fn medio_from_db(medio: &str) -> MedioDB::Model {
    let db = Database::connect("sqlite://db.sqlite?mode=ro")
        .await
        .unwrap();
    MedioDB::Entity::find()
        .filter(MedioDB::Column::Medio.eq(medio))
        .one(&db)
        .await
        .unwrap()
        .unwrap()
}
impl Default for Pago {
    fn default() -> Self {
        let res = async_runtime::block_on(medio_from_db("Efectivo"));
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
        let medio_pago = MedioPago {
            medio: Arc::from(res.medio),
            id: res.id,
        };
        let int_id = random();
        Pago {
            medio_pago,
            monto: 0.0,
            int_id,
            pagado: 0.0,
        }
    }
}
<<<<<<< HEAD

pub async fn medio_from_db(medio: &str, db: &DatabaseConnection) -> MedioDB::Model {
    MedioDB::Entity::find()
        .filter(MedioDB::Column::Medio.eq(medio))
        .one(db)
        .await
        .unwrap()
        .unwrap()
}
=======
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded