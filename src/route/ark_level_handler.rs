use crate::{
    repository::ark_level_repository::ArkLevelInfo, MaaAppState, MaaResult,
};
use axum::extract::State;
use axum::Json;

pub async fn get_levels(
    state: State<MaaAppState>,
) -> MaaResult<Json<Vec<ArkLevelInfo>>> {
    let levels = state.ark_level_repository.query_all_levels().await?;
    Ok(Json(levels.into_iter().map(Into::into).collect()))
}
