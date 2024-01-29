//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pago")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub medio_pago: i64,
    #[sea_orm(column_type = "Double")]
    pub monto: f64,
    pub venta: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::medio_pago::Entity",
        from = "Column::MedioPago",
        to = "super::medio_pago::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MedioPago,
    #[sea_orm(has_many = "super::relacion_venta_pago::Entity")]
    RelacionVentaPago,
    #[sea_orm(
        belongs_to = "super::venta::Entity",
        from = "Column::Venta",
        to = "super::venta::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Venta,
}

impl Related<super::medio_pago::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MedioPago.def()
    }
}

impl Related<super::relacion_venta_pago::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionVentaPago.def()
    }
}

impl Related<super::venta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Venta.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
