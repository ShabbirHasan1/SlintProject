
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "producto")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Double")]
    pub precio_de_venta: f64,
    #[sea_orm(column_type = "Double")]
    pub porcentaje: f64,
    #[sea_orm(column_type = "Double")]
    pub precio_de_costo: f64,
    pub tipo_producto: String,
    pub marca: String,
    pub variedad: String,
    pub presentacion: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::codigo_barras::Entity")]
    CodigoBarras,
    #[sea_orm(has_many = "super::relacion_prod_prov::Entity")]
    RelacionProdProv,
    #[sea_orm(has_many = "super::relacion_venta_prod::Entity")]
    RelacionVentaProd,
}

impl Related<super::codigo_barras::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CodigoBarras.def()
    }
}

impl Related<super::relacion_prod_prov::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionProdProv.def()
    }
}

impl Related<super::relacion_venta_prod::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionVentaProd.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
