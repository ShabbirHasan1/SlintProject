//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "relacion_venta_rub")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub cantidad: u8,
    pub rubro: i64,
    pub venta: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::rubro::Entity",
        from = "Column::Rubro",
        to = "super::rubro::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Rubro,
    #[sea_orm(
        belongs_to = "super::venta::Entity",
        from = "Column::Venta",
        to = "super::venta::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Venta,
}

impl Related<super::rubro::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rubro.def()
    }
}

impl Related<super::venta::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Venta.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
