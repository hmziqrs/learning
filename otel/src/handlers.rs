use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::cache::RedisCache;
use crate::config::Config;
use crate::entity::todo;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodoRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub cache: RedisCache,
    pub config: Config,
}

#[instrument(skip(state))]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<todo::Model>, StatusCode> {
    let span = tracing::info_span!("create_todo", title = %payload.title);
    let _enter = span.enter();

    let new_todo = todo::ActiveModel {
        title: Set(payload.title),
        description: Set(payload.description),
        ..Default::default()
    };

    match new_todo.save(&state.db).await {
        Ok(result) => {
            let todo_model = result.try_into_model().map_err(|e| {
                error!("Failed to convert active model: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            if let Err(e) = state.cache.set_todo(&todo_model) {
                error!("Failed to cache todo: {:?}", e);
            } else {
                info!("Cached todo with ID: {}", todo_model.id);
            }

            info!("Created todo with ID: {}", todo_model.id);
            Ok(Json(todo_model))
        }
        Err(e) => {
            error!("Failed to create todo: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[instrument(skip(state))]
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<todo::Model>, StatusCode> {
    let span = tracing::info_span!("get_todo", id = %id);
    let _enter = span.enter();

    match state.cache.get_todo(&id) {
        Ok(Some(todo_model)) => {
            info!("Cache hit for todo ID: {}", id);
            Ok(Json(todo_model))
        }
        Ok(None) => {
            info!("Cache miss for todo ID: {}", id);
            
            match todo::Entity::find_by_id(id).one(&state.db).await {
                Ok(Some(todo_model)) => {
                    if let Err(e) = state.cache.set_todo(&todo_model) {
                        error!("Failed to cache todo: {:?}", e);
                    } else {
                        info!("Cached todo with ID: {}", todo_model.id);
                    }
                    Ok(Json(todo_model))
                }
                Ok(None) => {
                    info!("Todo not found with ID: {}", id);
                    Err(StatusCode::NOT_FOUND)
                }
                Err(e) => {
                    error!("Failed to fetch todo: {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            error!("Failed to check cache: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[instrument(skip(state))]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<todo::Model>, StatusCode> {
    let span = tracing::info_span!("update_todo", id = %id);
    let _enter = span.enter();

    match todo::Entity::find_by_id(id).one(&state.db).await {
        Ok(Some(todo_model)) => {
            let mut active_model: todo::ActiveModel = todo_model.into();
            
            if let Some(title) = payload.title {
                active_model.title = Set(title);
            }
            if let Some(description) = payload.description {
                active_model.description = Set(description);
            }
            if let Some(completed) = payload.completed {
                active_model.completed = Set(completed);
            }

            match active_model.update(&state.db).await {
                Ok(updated_todo) => {
                    if let Err(e) = state.cache.set_todo(&updated_todo) {
                        error!("Failed to update cache: {:?}", e);
                    } else {
                        info!("Updated cache for todo ID: {}", updated_todo.id);
                    }
                    info!("Updated todo with ID: {}", updated_todo.id);
                    Ok(Json(updated_todo))
                }
                Err(e) => {
                    error!("Failed to update todo: {:?}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => {
            info!("Todo not found for update with ID: {}", id);
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Failed to find todo for update: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[instrument(skip(state))]
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let span = tracing::info_span!("delete_todo", id = %id);
    let _enter = span.enter();

    match todo::Entity::delete_by_id(id).exec(&state.db).await {
        Ok(res) => {
            if res.rows_affected > 0 {
                if let Err(e) = state.cache.delete_todo(&id) {
                    error!("Failed to delete from cache: {:?}", e);
                } else {
                    info!("Deleted from cache todo ID: {}", id);
                }
                info!("Deleted todo with ID: {}", id);
                Ok(StatusCode::NO_CONTENT)
            } else {
                info!("No todo found to delete with ID: {}", id);
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            error!("Failed to delete todo: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn todo_routes() -> Router<AppState> {
    Router::new()
        .route("/todos", post(create_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", delete(delete_todo))
}