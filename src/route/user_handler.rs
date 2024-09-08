use std::sync::Arc;

use axum::extract::{Json, State};
use axum::routing::post;
use axum::Router;
use axum_macros::debug_handler;

use crate::{AppState, MaaAppState, MaaResult};

use super::{request::user::RegisterRequest, response::user::MaaUserInfo};

pub fn get_user_router() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register))
}

#[debug_handler]
async fn register(
    state: State<MaaAppState>,
    Json(req): Json<RegisterRequest>,
) -> MaaResult<Json<MaaUserInfo>> {
    state.user_service.register(req).await.map(Json)
}
