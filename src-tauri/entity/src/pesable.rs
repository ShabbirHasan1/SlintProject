use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "pesable")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub codigo: i64,
    #[sea_orm(column_type = "Double")]
    pub precio_peso: f64,
    #[sea_orm(column_type = "Double")]
    pub porcentaje: f64,
    #[sea_orm(column_type = "Double")]
    pub costo_kilo: f64,
    pub descripcion: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::relacion_venta_pes::Entity")]
    RelacionVentaPes,
}

impl Related<super::relacion_venta_pes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionVentaPes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
