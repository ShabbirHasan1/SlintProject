use chrono::NaiveDateTime;
use entity::prelude::{CliDB, DeudaDB, PagoDB, VentaDB};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

<<<<<<< HEAD
use super::{AppError, Mapper, Res, User, Venta};

=======
use super::{error::AppError, lib::Mapper, user::User, venta::Venta};
type Res<T> = std::result::Result<T, AppError>;
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
#[derive(Serialize, Clone, Debug, Deserialize)]
pub enum Cliente {
    Final,
    Regular(Cli),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cli {
    id: i32,
    nombre: Arc<str>,
    dni: i32,
<<<<<<< HEAD
=======
    credito: bool,
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
    activo: bool,
    created: NaiveDateTime,
    limite: Cuenta,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Cuenta {
<<<<<<< HEAD
    Auth(f32),
=======
    Auth(Option<f32>),
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
    Unauth,
}
impl Cli {
    pub async fn new_to_db(
        db: &DatabaseConnection,
        nombre: &str,
        dni: i32,
<<<<<<< HEAD
=======
        credito: bool,
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f32>,
    ) -> Res<Cli> {
        match CliDB::Entity::find()
            .filter(CliDB::Column::Dni.eq(dni))
            .one(db)
            .await?
        {
            Some(_) => {
                return Err(AppError::ExistingError {
                    objeto: "Cliente".to_string(),
<<<<<<< HEAD
                    instancia: dni.to_string(),
=======
                    instancia: format!("{}", dni),
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
                })
            }
            None => {
                let model = CliDB::ActiveModel {
                    nombre: Set(nombre.to_string()),
                    dni: Set(dni),
<<<<<<< HEAD
=======
                    credito: Set(credito),
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
                    activo: Set(activo),
                    created: Set(created),
                    limite: Set(limite),
                    ..Default::default()
                };
                let res = CliDB::Entity::insert(model).exec(db).await?;
                Ok(Cli {
                    id: res.last_insert_id,
                    nombre: Arc::from(nombre),
                    dni,
                    activo,
<<<<<<< HEAD
                    created,
                    limite: match limite {
                        Some(limit) => Cuenta::Auth(limit),
                        None => Cuenta::Unauth,
=======
                    credito,
                    created,
                    limite: match credito {
                        true => Cuenta::Auth(limite),
                        false => Cuenta::Unauth,
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
                    },
                })
            }
        }
    }
    pub fn new(
        id: i32,
        nombre: Arc<str>,
        dni: i32,
<<<<<<< HEAD
=======
        credito: bool,
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
        activo: bool,
        created: NaiveDateTime,
        limite: Option<f32>,
    ) -> Cli {
        Cli {
            id,
            nombre,
            dni,
<<<<<<< HEAD
            limite: match limite {
                Some(limit) => Cuenta::Auth(limit),
                None => Cuenta::Unauth,
=======
            credito,
            limite: match credito {
                true => Cuenta::Auth(limite),
                false => Cuenta::Unauth,
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
            },
            activo,
            created,
        }
    }
    pub fn id(&self) -> &i32 {
        &self.id
    }
<<<<<<< HEAD
    #[cfg(test)]
    pub fn dni(&self) -> &i32 {
        &self.dni
    }
    pub fn limite(&self) -> &Cuenta {
        &self.limite
    }
    #[cfg(test)]
    pub fn nombre(&self) -> &str {
        self.nombre.as_ref()
=======
    pub fn credito(&self) -> &bool {
        &self.credito
>>>>>>> 21fee32d71e5a50e82c19600e3d108291a849ded
    }
    pub async fn get_deuda(&self, db: &DatabaseConnection) -> Res<f32> {
        Ok(DeudaDB::Entity::find()
            .select_only()
            .column(DeudaDB::Column::Monto)
            .filter(Condition::all().add(DeudaDB::Column::Cliente.eq(self.id)))
            .all(db)
            .await?
            .iter()
            .map(|m| m.monto)
            .sum::<f32>())
    }
    pub async fn get_deuda_detalle(
        &self,
        db: &DatabaseConnection,
        user: Option<Arc<User>>,
    ) -> Res<Vec<Venta>> {
        let mut ventas = Vec::new();
        let models = VentaDB::Entity::find()
            .filter(
                Condition::all()
                    .add(VentaDB::Column::Cliente.eq(self.id))
                    .add(VentaDB::Column::Paga.eq(false)),
            )
            .all(db)
            .await?;
        for model in models {
            ventas.push(Mapper::map_model_sale(&model, db, &user).await?);
        }
        Ok(ventas)
    }

    pub async fn pagar_deuda_especifica(
        id: i32,
        db: &DatabaseConnection,
        venta: Venta,
        user: &Option<Arc<User>>,
    ) -> Res<Venta> {
        let model = match VentaDB::Entity::find_by_id(*venta.id()).one(db).await? {
            Some(model) => model,
            None => return Err(AppError::IncorrectError(String::from("Id inexistente"))),
        };
        match model.cliente {
            Some(cli) => {
                if cli == id {
                    let mut model = model.clone().into_active_model();
                    model.paga = Set(true);
                    model.update(db).await?;
                } else {
                    return Err(AppError::IncorrectError("Cliente Incorrecto".to_string()));
                }
            }
            None => return Err(AppError::IncorrectError(String::from("Cliente Incorrecto"))),
        }
        let venta = Mapper::map_model_sale(&model, db, &user).await?;
        Ok(venta)
    }
    pub async fn pagar_deuda_general(id: i32, db: &DatabaseConnection, mut monto: f32) -> Res<f32> {
        let models = VentaDB::Entity::find()
            .filter(
                Condition::all()
                    .add(VentaDB::Column::Cliente.eq(id))
                    .add(VentaDB::Column::Paga.eq(false)),
            )
            .order_by_asc(VentaDB::Column::Time)
            .all(db)
            .await?;
        println!("{:#?} encontrados {}", models, models.len());
        let resto = monto
            - models
                .iter()
                .map(|model| model.monto_total - model.monto_pagado)
                .sum::<f32>();
        for model in models {
            if monto <= 0.0 {
                break;
            }
            let mut model = model.into_active_model();
            let mut pagos = PagoDB::Entity::find()
                .filter(
                    Condition::all()
                        .add(PagoDB::Column::Venta.eq(model.id.clone().unwrap()))
                        .add(PagoDB::Column::MedioPago.eq(0)),
                )
                .all(db)
                .await?
                .iter()
                .cloned()
                .map(|pago| pago.into_active_model())
                .collect::<Vec<PagoDB::ActiveModel>>();
            let mut completados: u8 = 0;
            for i in 0..pagos.len() {
                if monto <= 0.0 {
                    break;
                }
                if pagos[i].pagado.as_ref() < pagos[i].monto.as_ref() {
                    if monto >= pagos[i].monto.as_ref() - pagos[i].pagado.as_ref() {
                        monto -= pagos[i].monto.as_ref() - pagos[i].pagado.as_ref();
                        pagos[i].pagado = Set(*pagos[i].monto.as_ref());
                        completados += 1;
                        pagos[i].clone().update(db).await?;
                    } else {
                        pagos[i].pagado = Set(pagos[i].pagado.as_ref() + monto);
                        monto = 0.0;
                        pagos[i].clone().update(db).await?;
                    }
                }
            }
            if completados == pagos.len() as u8 {
                model.paga = Set(true);
                model.update(db).await?;
            }
        }
        Ok(resto)
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