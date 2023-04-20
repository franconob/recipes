use axum::{extract::State, Json};
use sea_orm::entity::*;

use crate::{AppState, DbError};
use entity::ingredient::{Entity, Model as Ingredient};

pub async fn list_ingredients(
    State(state): State<AppState>,
) -> Result<Json<Vec<Ingredient>>, DbError> {
    Ok(Json(Entity::find().all(&state.conn).await?))
}
