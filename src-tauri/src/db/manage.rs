use dotenvy::dotenv;

use sqlx::{Executor, Pool, Sqlite};
use tauri::async_runtime::{block_on, spawn};

pub fn fresh(db: &Pool<Sqlite>) {
    down(db);
    up(db);
}

pub fn down(db: &Pool<Sqlite>) {
    dotenv().ok();
    block_on(db.execute(sqlx::query(
        "drop table if exists cajas
        ",
    )))
    .unwrap();
}
pub fn up(db: &Pool<Sqlite>) {
    dotenv().ok();
    spawn(db.execute(sqlx::query!(
    "CREATE TABLE IF NOT EXISTS cajas (
            id integer PRIMARY KEY AUTOINCREMENT not null,
            inicio datetime not null,
            cierre datetime,
            monto_inicio real not null,
            monto_cierre real,
            ventas_totales real not null,
            cajero string
        )",)));
    spawn(db.execute(sqlx::query!(
    "CREATE TABLE IF NOT EXISTS clientes (
            id integer PRIMARY KEY AUTOINCREMENT not null,
            nombre string not null,
            dni integer not null,
            limite real,
            activo boolean not null,
            time datetime not null
        )",)));
}
enum Config {
    Table,
    Id,
    PoliticaRedondeo,
    FormatoProducto,
    ModoMayus,
    CantidadProductos,
}

pub enum MedioPago {
    Table,
    Id,
    Medio,
}


enum CodigoBarras {
    Table,
    Id,
    Codigo,
    Producto,
}


enum Deuda {
    Table,
    Id,
    Cliente,
    Pago,
    Monto,
}



enum Movimiento {
    Table,
    Id,
    Caja,
    Tipo,
    Monto,
    Descripcion,
    Time,
}

pub enum Pago {
    Table,
    Id,
    MedioPago,
    Monto,
    Pagado,
    Venta,
}

pub enum Pesable {
    Table,
    Id,
    Codigo,
    PrecioPeso,
    Porcentaje,
    CostoKilo,
    Descripcion,
    UpdatedAt,
}

pub enum Producto {
    Table,
    Id,
    PrecioDeVenta,
    Porcentaje,
    PrecioDeCosto,
    TipoProducto,
    Marca,
    Variedad,
    Presentacion,
    Cantidad,
    UpdatedAt,
}

pub enum Proveedor {
    Table,
    Id,
    Nombre,
    Contacto,
    UpdatedAt,
}

enum RelacionProdProv {
    Table,
    Id,
    Producto,
    Proveedor,
    Codigo,
}

enum RelacionVentaPes {
    Table,
    Id,
    Cantidad,
    Precio,
    Pesable,
    Venta,
}

enum RelacionVentaProd {
    Table,
    Id,
    Cantidad,
    Precio,
    Producto,
    Venta,
}

enum RelacionVentaRub {
    Table,
    Id,
    Cantidad,
    Rubro,
    Precio,
    Venta,
}

pub enum Rubro {
    Table,
    Id,
    Codigo,
    Monto,
    Descripcion,
    UpdatedAt,
}

enum User {
    Table,
    Id,
    UserId,
    Nombre,
    Pass,
    Rango,
}

pub enum Venta {
    Table,
    Id,
    Time,
    MontoTotal,
    MontoPagado,
    Cliente,
    Cerrada,
    Paga,
    Pos,
}

    
    // pub enum Caja {
    //     Table,
    //     Id,
    //     Inicio,
    //     Cierre,
    //     MontoInicio,
    //     MontoCierre,
    //     VentasTotales,
    //     Cajero,
    // }