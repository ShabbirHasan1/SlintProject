use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use std::sync::Arc;

use super::error::AppError;
type Res<T> = std::result::Result<T, AppError>;
#[derive(Serialize, Clone, Debug)]
pub enum Cliente {
    Final,
    Regular(Cli),
}

#[derive(Serialize, Clone, Debug)]
pub struct Cli {
    id: i64,
    nombre: Arc<str>,
    dni: i64,
    credito: bool,
    activo: bool,
    created: NaiveDateTime,
    limite: Cuenta,
}
#[derive(Serialize, Clone, Debug)]
pub enum Cuenta {
    Auth(Option<f64>),
    Unauth,
}
impl Cli {
    pub async fn new_to_db(
        db: &DatabaseConnection,
        nombre: &str,
        dni: i64,
        credito: bool,
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f64>,
    ) -> Res<Cli> {
        match entity::cliente::Entity::find()
            .filter(entity::cliente::Column::Dni.eq(dni))
            .one(db)
            .await?
        {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: "Cliente".to_string(),
                    instancia: format!("{}", dni),
                })
            }
            None => {
                let model = entity::cliente::ActiveModel {
                    nombre: Set(nombre.to_string()),
                    dni: Set(dni),
                    credito: Set(credito),
                    activo: Set(activo),
                    created: Set(created),
                    limite: Set(limite),
                    ..Default::default()
                };
                let res = entity::cliente::Entity::insert(model).exec(db).await?;
                Ok(Cli {
                    id: res.last_insert_id,
                    nombre: Arc::from(nombre),
                    dni,
                    activo,
                    credito,
                    created,
                    limite: match credito {
                        true => Cuenta::Auth(limite),
                        false => Cuenta::Unauth,
                    },
                })
            }
        }
    }
    pub fn new(
        id: i64,
        nombre: Arc<str>,
        dni: i64,
        credito: bool,
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f64>,
    ) -> Cli {
        Cli {
            id,
            nombre,
            dni,
            credito,
            limite: match credito {
                true => Cuenta::Auth(limite),
                false => Cuenta::Unauth,
            },
            activo,
            created,
        }
    }
    pub fn id(&self) -> &i64 {
        &self.id
    }
    pub fn credito(&self) -> &bool {
        &self.credito
    }
    pub async fn get_deuda(&self, db: &DatabaseConnection) -> Res<f64> {
        let res = entity::venta::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::venta::Column::Cliente.eq(Some(self.id)))
                    .add(entity::venta::Column::Cerrada.eq(false)),
            )
            .all(db)
            .await?
            .iter()
            .map(|x| x.id)
            .collect::<Vec<i64>>();

        let pagos = entity::pago::Entity::find()
            .filter(entity::pago::Column::Venta.is_in(res))
            .all(db)
            .await?;
        // let medio=entity::medio_pago::Entity::find().filter(entity::medio_pago::Column::Medio.eq(v))

        Ok(0.0)
    }
}

impl<'a> Cliente {
    pub fn new(cli: Option<Cli>) -> Cliente {
        match cli {
            Some(a) => Cliente::Regular(a),
            None => Cliente::Final,
        }
    }
}
impl Default for Cliente {
    fn default() -> Self {
        Cliente::Final
    }
}
