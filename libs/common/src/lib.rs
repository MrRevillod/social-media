pub mod database;
#[allow(dead_code, unused_imports, clippy::err_expect)]
mod prisma;
mod repositories;

pub async fn grettings() {
    println!("Hello from common lib");
}
