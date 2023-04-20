use axum::extract::Path;
use axum::{extract::State, Json};
use axum_macros;
use axum_macros::debug_handler;
use sea_orm::entity::prelude::*;

use entity::unit::Entity as UnitEntity;
use entity::unit::Model as UnitModel;

use crate::AppState;
use crate::DbError;

#[debug_handler]
pub async fn get_units(state: State<AppState>) -> Result<Json<Vec<UnitModel>>, DbError> {
    Ok(Json(UnitEntity::find().all(&state.conn).await?))
}

#[debug_handler]
pub async fn get_unit(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UnitModel>, DbError> {
    UnitEntity::find_by_id(id)
        .one(&state.conn)
        .await?
        .map(|v| Json(v))
        .ok_or(DbError::NotFoundError("Unit".to_owned(), id))
}
