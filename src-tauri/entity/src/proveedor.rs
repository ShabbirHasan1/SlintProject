use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "proveedor")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub updated_at: NaiveDateTime,
    pub nombre: String,
    pub contacto: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::relacion_prod_prov::Entity")]
    RelacionProdProv,
}

impl Related<super::relacion_prod_prov::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionProdProv.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
