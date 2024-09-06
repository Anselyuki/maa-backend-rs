use axum::Json;

use crate::{repository::ark_level_repository::ArkLevelInfo, MaaAppState, MaaResult};

pub async fn get_levels(state: MaaAppState) -> MaaResult<Json<Vec<ArkLevelInfo>>> {
    let levels = state.ark_level_repository.query_all_levels().await?;
    Ok(Json(levels.into_iter().map(Into::into).collect()))
}
