//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "relacion_venta_pago")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub pago: i64,
    pub venta: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::pago::Entity",
        from = "Column::Pago",
        to = "super::pago::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Pago,
    #[sea_orm(
        belongs_to = "super::venta::Entity",
        from = "Column::Venta",
        to = "super::venta::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Venta,
}

impl Related<super::pago::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Pago.def()
    }
}

impl Related<super::venta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Venta.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
