//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;

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
    pub cliente: Option<i64>,
    pub cerrada: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::cliente::Entity",
        from = "Column::Cliente",
        to = "super::cliente::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Cliente,
    #[sea_orm(has_many = "super::pago::Entity")]
    Pago,
    #[sea_orm(has_many = "super::relacion_venta_pes::Entity")]
    RelacionVentaPes,
    #[sea_orm(has_many = "super::relacion_venta_prod::Entity")]
    RelacionVentaProd,
    #[sea_orm(has_many = "super::relacion_venta_rub::Entity")]
    RelacionVentaRub,
}

impl Related<super::cliente::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cliente.def()
    }
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
