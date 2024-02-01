use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Proveedor::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Proveedor::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Proveedor::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Proveedor::Nombre).string().not_null())
                    .col(ColumnDef::new(Proveedor::Contacto).big_integer())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Venta::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Venta::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Venta::MontoTotal).double().not_null())
                    .col(ColumnDef::new(Venta::MontoPagado).double().not_null())
                    .col(ColumnDef::new(Venta::Time).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Producto::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Producto::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Producto::PrecioDeVenta).double().not_null())
                    .col(ColumnDef::new(Producto::Porcentaje).double().not_null())
                    .col(ColumnDef::new(Producto::PrecioDeCosto).double().not_null())
                    .col(ColumnDef::new(Producto::TipoProducto).string().not_null())
                    .col(ColumnDef::new(Producto::Marca).string().not_null())
                    .col(ColumnDef::new(Producto::Variedad).string().not_null())
                    .col(ColumnDef::new(Producto::Presentacion).string().not_null())
                    .col(ColumnDef::new(Producto::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(CodigoBarras::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CodigoBarras::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CodigoBarras::Codigo)
                            .big_integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(CodigoBarras::Producto)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("producto_fk")
                            .from(CodigoBarras::Table, CodigoBarras::Producto)
                            .to(Producto::Table, Producto::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Pesable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pesable::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pesable::Codigo).big_integer().not_null())
                    .col(ColumnDef::new(Pesable::PrecioPeso).double().not_null())
                    .col(ColumnDef::new(Pesable::Porcentaje).double().not_null())
                    .col(ColumnDef::new(Pesable::CostoKilo).double().not_null())
                    .col(ColumnDef::new(Pesable::Descripcion).string().not_null())
                    .col(ColumnDef::new(Pesable::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Rubro::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Rubro::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Rubro::Monto).double().not_null())
                    .col(ColumnDef::new(Rubro::Descripcion).string().not_null())
                    .col(ColumnDef::new(Rubro::UpdatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Pago::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Pago::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Pago::MedioPago).big_integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("medio_pago_fk")
                            .from(Pago::Table, Pago::MedioPago)
                            .to(MedioPago::Table, MedioPago::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Pago::Monto).double().not_null())
                    .col(ColumnDef::new(Pago::Venta).big_integer().not_null())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("venta-fk")
                            .from(Pago::Table, Pago::Venta)
                            .to(Venta::Table, Venta::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(RelacionVentaPes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelacionVentaPes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaPes::Cantidad)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaPes::Pesable)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("pesable_fk")
                            .from(RelacionVentaPes::Table, RelacionVentaPes::Pesable)
                            .to(Pesable::Table, Pesable::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaPes::Venta)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("venta_fk")
                            .from(RelacionVentaPes::Table, RelacionVentaPes::Venta)
                            .to(Venta::Table, Venta::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(RelacionVentaRub::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelacionVentaRub::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaRub::Cantidad)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaRub::Rubro)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("rubro_fk")
                            .from(RelacionVentaRub::Table, RelacionVentaRub::Rubro)
                            .to(Rubro::Table, Rubro::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaRub::Venta)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("venta_fk")
                            .from(RelacionVentaRub::Table, RelacionVentaRub::Venta)
                            .to(Venta::Table, Venta::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(RelacionVentaProd::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelacionVentaProd::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaProd::Producto)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("producto_fk")
                            .from(RelacionVentaProd::Table, RelacionVentaProd::Producto)
                            .to(Producto::Table, Producto::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaProd::Cantidad)
                            .small_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RelacionVentaProd::Venta)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("venta_fk")
                            .from(RelacionVentaProd::Table, RelacionVentaProd::Venta)
                            .to(Venta::Table, Venta::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(RelacionProdProv::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RelacionProdProv::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RelacionProdProv::Producto)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("producto_fk")
                            .from(RelacionProdProv::Table, RelacionProdProv::Producto)
                            .to(Producto::Table, Producto::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RelacionProdProv::Proveedor)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("proveedor_fk")
                            .from(RelacionProdProv::Table, RelacionProdProv::Proveedor)
                            .to(Proveedor::Table, Proveedor::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(RelacionProdProv::Codigo).big_integer())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(MedioPago::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MedioPago::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MedioPago::Medio).string().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Config::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Config::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Config::CantidadProductos)
                            .small_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Config::FormatoProducto).string().not_null())
                    .col(ColumnDef::new(Config::ModoMayus).string().not_null())
                    .col(ColumnDef::new(Config::PoliticaRedondeo).double().not_null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Caja::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Caja::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Caja::Inicio).string().not_null())
                    .col(ColumnDef::new(Caja::Cierre).string())
                    .col(ColumnDef::new(Caja::MontoInicio).double().not_null())
                    .col(ColumnDef::new(Caja::MontoCierre).double())
                    .col(ColumnDef::new(Caja::VentasTotales).double().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Proveedor::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Venta::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Producto::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CodigoBarras::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Pesable::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Rubro::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Pago::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RelacionVentaProd::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RelacionProdProv::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Proveedor {
    Table,
    Id,
    Nombre,
    Contacto,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum Venta {
    Table,
    Id,
    Time,
    MontoTotal,
    MontoPagado,
}
#[derive(DeriveIden)]
enum Producto {
    Table,
    Id,
    PrecioDeVenta,
    Porcentaje,
    PrecioDeCosto,
    TipoProducto,
    Marca,
    Variedad,
    Presentacion,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum CodigoBarras {
    Table,
    Id,
    Codigo,
    Producto,
}
#[derive(DeriveIden)]
enum Pesable {
    Table,
    Id,
    Codigo,
    PrecioPeso,
    Porcentaje,
    CostoKilo,
    Descripcion,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum Rubro {
    Table,
    Id,
    Monto,
    Descripcion,
    UpdatedAt,
}
#[derive(DeriveIden)]
enum Pago {
    Table,
    Id,
    MedioPago,
    Monto,
    Venta,
}

#[derive(DeriveIden)]
enum RelacionVentaProd {
    Table,
    Id,
    Cantidad,
    Producto,
    Venta,
}
#[derive(DeriveIden)]
enum RelacionVentaPes {
    Table,
    Id,
    Cantidad,
    Pesable,
    Venta,
}
#[derive(DeriveIden)]
enum RelacionVentaRub {
    Table,
    Id,
    Cantidad,
    Rubro,
    Venta,
}
#[derive(DeriveIden)]
enum RelacionProdProv {
    Table,
    Id,
    Producto,
    Proveedor,
    Codigo,
}
#[derive(DeriveIden)]
enum MedioPago {
    Table,
    Id,
    Medio,
}
#[derive(DeriveIden)]
enum Config {
    Table,
    Id,
    PoliticaRedondeo,
    FormatoProducto,
    ModoMayus,
    CantidadProductos,
}
#[derive(DeriveIden)]
enum Caja {
    Table,
    Id,
    Inicio,
    Cierre,
    MontoInicio,
    MontoCierre,
    VentasTotales,
}
