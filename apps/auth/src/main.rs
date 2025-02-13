#[tokio::main]
async fn main() {
    let _db = common::init_prisma_client().await;
}
