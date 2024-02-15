
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "venta")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Double")]
    pub monto_total: f64,
    #[sea_orm(column_type = "Double")]
    pub monto_pagado: f64,
    pub time: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::pago::Entity")]
    Pago,
    #[sea_orm(has_many = "super::relacion_venta_pes::Entity")]
    RelacionVentaPes,
    #[sea_orm(has_many = "super::relacion_venta_prod::Entity")]
    RelacionVentaProd,
    #[sea_orm(has_many = "super::relacion_venta_rub::Entity")]
    RelacionVentaRub,
}

impl Related<super::pago::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pago.def()
    }
}

impl Related<super::relacion_venta_pes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionVentaPes.def()
    }
}

impl Related<super::relacion_venta_prod::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionVentaProd.def()
    }
}

impl Related<super::relacion_venta_rub::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionVentaRub.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
