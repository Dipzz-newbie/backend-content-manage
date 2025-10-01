use sqlx::MySqlPool;
use uuid::Uuid;
use crate::{
    errors::{AppError, AppResult},
    models::*,
};

pub async fn register(pool: &MySqlPool, req: RegisterRequest) -> AppResult<UserResponse> {
    // Check if username already exists
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_one(pool)
        .await?;

    if count.0 > 0 {
        return Err(AppError::BadRequest("Username already exists".to_string()));
    }

    // Hash password
    let hashed_password = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|_| AppError::Internal)?;

    // Insert user
    sqlx::query(
        "INSERT INTO users (username, password, name, token) VALUES (?, ?, ?, NULL)"
    )
    .bind(&req.username)
    .bind(&hashed_password)
    .bind(&req.name)
    .execute(pool)
    .await?;

    Ok(UserResponse {
        username: req.username,
        name: req.name,
    })
}

pub async fn login(pool: &MySqlPool, req: LoginRequest) -> AppResult<LoginResponse> {
    // Find user
    let user = sqlx::query_as::<_, User>(
        "SELECT username, password, name, token FROM users WHERE username = ?"
    )
    .bind(&req.username)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    // Verify password
    let valid = bcrypt::verify(&req.password, &user.password)
        .map_err(|_| AppError::Internal)?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    // Generate token
    let token = Uuid::new_v4().to_string();

    // Update token
    sqlx::query("UPDATE users SET token = ? WHERE username = ?")
        .bind(&token)
        .bind(&user.username)
        .execute(pool)
        .await?;

    Ok(LoginResponse { token })
}

pub async fn update(
    pool: &MySqlPool,
    username: &str,
    req: UpdateUserRequest,
) -> AppResult<UserResponse> {
    // Check if user exists
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    if count.0 != 1 {
        return Err(AppError::NotFound("user is not found".to_string()));
    }

    // Build update query dynamically
    let mut updates = Vec::new();
    let mut query = String::from("UPDATE users SET ");

    if let Some(name) = &req.name {
        updates.push(format!("name = '{}'", name));
    }

    if let Some(password) = &req.password {
        let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)
            .map_err(|_| AppError::Internal)?;
        updates.push(format!("password = '{}'", hashed));
    }

    if updates.is_empty() {
        // No updates, just return current user
        let user = sqlx::query_as::<_, User>(
            "SELECT username, password, name, token FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_one(pool)
        .await?;
        
        return Ok(UserResponse {
            username: user.username,
            name: user.name,
        });
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(" WHERE username = '{}'", username));

    sqlx::query(&query).execute(pool).await?;

    // Fetch updated user
    let user = sqlx::query_as::<_, User>(
        "SELECT username, password, name, token FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_one(pool)
    .await?;

    Ok(UserResponse {
        username: user.username,
        name: user.name,
    })
}

pub async fn logout(pool: &MySqlPool, username: &str) -> AppResult<()> {
    let result = sqlx::query("UPDATE users SET token = NULL WHERE username = ?")
        .bind(username)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("user is not found".to_string()));
    }

    Ok(())
}