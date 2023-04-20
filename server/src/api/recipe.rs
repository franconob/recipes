use axum::{extract::State, Json};
use axum_macros::debug_handler;
use sea_orm::{entity::prelude::*, ActiveModelTrait, Set};

use crate::{AppState, DbError};
use entity::recipe::{ActiveModel, Entity, Model};
use entity::recipe_ingredient::{Entity as RecipeIngredientEntity, Model as RecipeIngredient};

#[debug_handler]
pub async fn create_recipe(
    State(state): State<AppState>,
    Json(recipe_data): Json<Model>,
) -> Result<Json<Model>, DbError> {
    let model = ActiveModel {
        name: Set(recipe_data.name),
        ..Default::default()
    };

    Ok(Json(model.insert(&state.conn).await?))
}

pub async fn list_recipies(
    State(state): State<AppState>,
) -> Result<Json<Vec<(Model, Vec<RecipeIngredient>)>>, DbError> {
    Ok(Json(
        Entity::find()
            .find_with_related(RecipeIngredientEntity)
            .all(&state.conn)
            .await?,
    ))
}
