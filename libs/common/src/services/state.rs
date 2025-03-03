use crate::database::PgPoolRef;

#[derive(Clone, Debug)]
pub struct AppState {
    pub prisma: PgPoolRef,
}

impl AppState {
    pub fn new(prisma: PgPoolRef) -> Self {
        Self { prisma }
    }
}
