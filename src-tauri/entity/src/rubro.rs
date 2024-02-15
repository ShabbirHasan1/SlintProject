
use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "rubro")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub codigo: i64,
    #[sea_orm(column_type = "Double", nullable)]
    pub monto: Option<f64>,
    pub descripcion: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::relacion_venta_rub::Entity")]
    RelacionVentaRub,
}

impl Related<super::relacion_venta_rub::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RelacionVentaRub.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
