use axum::routing::post;
use axum::{routing::get, Router};
use sea_orm::SqlxPostgresConnector;
use sqlx::PgPool;

use server::api::{ingredient, recipe, unit};
use server::AppState;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:5432/postgres"
    )]
    pool: PgPool,
) -> shuttle_axum::ShuttleAxum {
    let conn = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);

    // run migrations

    let shared_state = AppState { conn };

    let router = Router::new()
        .route("/unit", get(unit::get_units))
        .route("/unit/:id", get(unit::get_unit))
        .route("/ingredient", get(ingredient::list_ingredients))
        .route(
            "/recipe",
            get(recipe::list_recipies).post(recipe::create_recipe),
        )
        .with_state(shared_state);

    Ok(router.into())
}
