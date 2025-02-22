use crate::database::postgres;
use std::sync::Arc;

pub type AppStateRef = Arc<AppState>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub prisma: postgres::PgPoolRef,
}

impl AppState {
    pub fn new(prisma: postgres::PgPoolRef) -> AppStateRef {
        Arc::new(Self { prisma })
    }
}
