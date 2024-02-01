//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.12

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "relacion_prod_prov")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub producto: i64,
    pub proveedor: i64,
    pub codigo: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::producto::Entity",
        from = "Column::Producto",
        to = "super::producto::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Producto,
    #[sea_orm(
        belongs_to = "super::proveedor::Entity",
        from = "Column::Proveedor",
        to = "super::proveedor::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Proveedor,
}

impl Related<super::producto::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Producto.def()
    }
}

impl Related<super::proveedor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Proveedor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
