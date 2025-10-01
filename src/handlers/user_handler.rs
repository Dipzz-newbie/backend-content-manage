use axum::{
    extract::{Extension, State},
    Json,
};
use std::sync::Arc;
use crate::{
    database::AppState,
    errors::{AppError, AppResult},
    models::*,
    services::user_service,
    validation::validate_request,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    validate_request(&req)?;
    let user = user_service::register(&state.pool, req).await?;
    Ok(Json(ApiResponse { data: user }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<ApiResponse<LoginResponse>>> {
    validate_request(&req)?;
    let response = user_service::login(&state.pool, req).await?;
    Ok(Json(ApiResponse { data: response }))
}

pub async fn get_current(
    Extension(user): Extension<User>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    Ok(Json(ApiResponse {
        data: UserResponse {
            username: user.username,
            name: user.name,
        },
    }))
}

pub async fn update(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    validate_request(&req)?;
    let updated_user = user_service::update(&state.pool, &user.username, req).await?;
    Ok(Json(ApiResponse { data: updated_user }))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<User>,
) -> AppResult<Json<ApiResponse<String>>> {
    user_service::logout(&state.pool, &user.username).await?;
    Ok(Json(ApiResponse { data: "OK".to_string() }))
}