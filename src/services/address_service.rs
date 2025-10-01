use sqlx::MySqlPool;
use crate::{
    errors::{AppError, AppResult},
    models::*,
};

async fn check_contact_exists(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
) -> AppResult<()> {
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM contacts WHERE id = ? AND username = ?"
    )
    .bind(contact_id)
    .bind(username)
    .fetch_one(pool)
    .await?;

    if count.0 != 1 {
        return Err(AppError::NotFound("contact is not found".to_string()));
    }

    Ok(())
}

pub async fn create(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
    req: CreateAddressRequest,
) -> AppResult<AddressResponse> {
    check_contact_exists(pool, username, contact_id).await?;

    let result = sqlx::query(
        "INSERT INTO addresses (street, city, province, country, postal_code, contact_id) 
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&req.street)
    .bind(&req.city)
    .bind(&req.province)
    .bind(&req.country)
    .bind(&req.postal_code)
    .bind(contact_id)
    .execute(pool)
    .await?;

    let id = result.last_insert_id() as i32;

    Ok(AddressResponse {
        id,
        street: req.street,
        city: req.city,
        province: req.province,
        country: req.country,
        postal_code: req.postal_code,
    })
}

pub async fn get(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
    address_id: i32,
) -> AppResult<AddressResponse> {
    check_contact_exists(pool, username, contact_id).await?;

    let address = sqlx::query_as::<_, Address>(
        "SELECT id, street, city, province, country, postal_code, contact_id 
         FROM addresses 
         WHERE id = ? AND contact_id = ?"
    )
    .bind(address_id)
    .bind(contact_id)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("address is not found".to_string()))?;

    Ok(address.into())
}

pub async fn update(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
    address_id: i32,
    req: UpdateAddressRequest,
) -> AppResult<AddressResponse> {
    check_contact_exists(pool, username, contact_id).await?;

    // Check if address exists
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM addresses WHERE id = ? AND contact_id = ?"
    )
    .bind(address_id)
    .bind(contact_id)
    .fetch_one(pool)
    .await?;

    if count.0 != 1 {
        return Err(AppError::NotFound("address is not found".to_string()));
    }

    // Update address
    sqlx::query(
        "UPDATE addresses 
         SET street = ?, city = ?, province = ?, country = ?, postal_code = ? 
         WHERE id = ?"
    )
    .bind(&req.street)
    .bind(&req.city)
    .bind(&req.province)
    .bind(&req.country)
    .bind(&req.postal_code)
    .bind(address_id)
    .execute(pool)
    .await?;

    Ok(AddressResponse {
        id: address_id,
        street: req.street,
        city: req.city,
        province: req.province,
        country: req.country,
        postal_code: req.postal_code,
    })
}

pub async fn remove(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
    address_id: i32,
) -> AppResult<()> {
    check_contact_exists(pool, username, contact_id).await?;

    let result = sqlx::query(
        "DELETE FROM addresses WHERE id = ? AND contact_id = ?"
    )
    .bind(address_id)
    .bind(contact_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("address is not found".to_string()));
    }

    Ok(())
}

pub async fn list(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
) -> AppResult<Vec<AddressResponse>> {
    check_contact_exists(pool, username, contact_id).await?;

    let addresses = sqlx::query_as::<_, Address>(
        "SELECT id, street, city, province, country, postal_code, contact_id 
         FROM addresses 
         WHERE contact_id = ?"
    )
    .bind(contact_id)
    .fetch_all(pool)
    .await?;

    Ok(addresses.into_iter().map(|a| a.into()).collect())
}