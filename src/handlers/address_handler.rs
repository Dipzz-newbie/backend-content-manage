use axum::{
    extract::{Extension, Path, State},
    Json,
};
use std::sync::Arc;
use crate::{
    database::AppState,
    errors::{AppError, AppResult},
    models::*,
    services::address_service,
    validation::validate_request,
};

pub async fn create(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(contact_id): Path<i32>,
    Json(req): Json<CreateAddressRequest>,
) -> AppResult<Json<ApiResponse<AddressResponse>>> {
    validate_request(&req)?;
    let address = address_service::create(&state.pool, &user.username, contact_id, req).await?;
    Ok(Json(ApiResponse { data: address }))
}

pub async fn get(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path((contact_id, address_id)): Path<(i32, i32)>,
) -> AppResult<Json<ApiResponse<AddressResponse>>> {
    let address = address_service::get(&state.pool, &user.username, contact_id, address_id).await?;
    Ok(Json(ApiResponse { data: address }))
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path((contact_id, address_id)): Path<(i32, i32)>,
    Json(req): Json<UpdateAddressRequest>,
) -> AppResult<Json<ApiResponse<AddressResponse>>> {
    validate_request(&req)?;
    let address = address_service::update(&state.pool, &user.username, contact_id, address_id, req).await?;
    Ok(Json(ApiResponse { data: address }))
}

pub async fn remove(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path((contact_id, address_id)): Path<(i32, i32)>,
) -> AppResult<Json<ApiResponse<String>>> {
    address_service::remove(&state.pool, &user.username, contact_id, address_id).await?;
    Ok(Json(ApiResponse { data: "OK".to_string() }))
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Path(contact_id): Path<i32>,
) -> AppResult<Json<ApiResponse<Vec<AddressResponse>>>> {
    let addresses = address_service::list(&state.pool, &user.username, contact_id).await?;
    Ok(Json(ApiResponse { data: addresses }))
}