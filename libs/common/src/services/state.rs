use crate::database::PgPoolRef;
use std::sync::Arc;

pub type AppStateRef = Arc<AppState>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub prisma: PgPoolRef,
}

impl AppState {
    pub fn new(prisma: PgPoolRef) -> AppStateRef {
        Arc::new(Self { prisma })
    }
}
