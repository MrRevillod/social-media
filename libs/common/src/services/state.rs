use axum::{
    extract::{FromRef, FromRequest, Request},
    response::Response,
};

use crate::database::postgre;
use std::sync::Arc;

pub type AppStateRef = Arc<AppState>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub prisma: postgre::DbConnectionRef,
}

impl AppState {
    pub fn new(prisma: postgre::DbConnectionRef) -> AppStateRef {
        Arc::new(Self { prisma })
    }
}

pub struct State {
    pub prisma: postgre::DbConnectionRef,
}

impl<S> FromRequest<S> for State
where
    S: Send + Sync,
    AppStateRef: FromRef<S>,
{
    type Rejection = Response;

    async fn from_request(_req: Request, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            prisma: AppStateRef::from_ref(state).prisma.clone(),
        })
    }
}
