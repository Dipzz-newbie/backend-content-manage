use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use std::sync::Arc;
use crate::{
    database::AppState,
    errors::{AppError, AppResult},
    models::*,
    services::contact_service,
    validation::validate_request,
};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(req): Json<CreateContactRequest>,
) -> AppResult<Json<ApiResponse<ContactResponse>>> {
    validate_request(&req)?;
    let contact = contact_service::create(&state.pool, &user.username, req).await?;
    Ok(Json(ApiResponse { data: contact }))
}

pub async fn get(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<i32>,
) -> AppResult<Json<ApiResponse<ContactResponse>>> {
    let contact = contact_service::get(&state.pool, &user.username, id).await?;
    Ok(Json(ApiResponse { data: contact }))
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateContactRequest>,
) -> AppResult<Json<ApiResponse<ContactResponse>>> {
    validate_request(&req)?;
    let contact = contact_service::update(&state.pool, &user.username, id, req).await?;
    Ok(Json(ApiResponse { data: contact }))
}

pub async fn remove(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(id): Path<i32>,
) -> AppResult<Json<ApiResponse<String>>> {
    contact_service::remove(&state.pool, &user.username, id).await?;
    Ok(Json(ApiResponse { data: "OK".to_string() }))
}

pub async fn search(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Query(req): Query<SearchContactRequest>,
) -> AppResult<Json<ContactSearchResponse>> {
    let result = contact_service::search(&state.pool, &user.username, req).await?;
    Ok(Json(result))
}