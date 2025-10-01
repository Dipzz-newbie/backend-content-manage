use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::{database::AppState, errors::AppError, models::User};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let headers = req.headers();
    let token = get_token_from_headers(headers)?;

    let user = sqlx::query_as::<_, User>(
        "SELECT username, password, name, token FROM users WHERE token = ?"
    )
    .bind(token)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

fn get_token_from_headers(headers: &HeaderMap) -> Result<&str, AppError> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)
}