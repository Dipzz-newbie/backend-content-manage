use sqlx::MySqlPool;
use crate::{
    errors::{AppError, AppResult},
    models::*,
};

pub async fn create(
    pool: &MySqlPool,
    username: &str,
    req: CreateContactRequest,
) -> AppResult<ContactResponse> {
    let result = sqlx::query(
        "INSERT INTO contacts (first_name, last_name, email, phone, username) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(username)
    .execute(pool)
    .await?;

    let id = result.last_insert_id() as i32;

    Ok(ContactResponse {
        id,
        first_name: req.first_name,
        last_name: req.last_name,
        email: req.email,
        phone: req.phone,
    })
}

pub async fn get(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
) -> AppResult<ContactResponse> {
    let contact = sqlx::query_as::<_, Contact>(
        "SELECT id, first_name, last_name, email, phone, username 
         FROM contacts 
         WHERE id = ? AND username = ?"
    )
    .bind(contact_id)
    .bind(username)
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::NotFound("contact is not found".to_string()))?;

    Ok(contact.into())
}

pub async fn update(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
    req: UpdateContactRequest,
) -> AppResult<ContactResponse> {
    // Check if contact exists
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

    // Update contact
    sqlx::query(
        "UPDATE contacts 
         SET first_name = ?, last_name = ?, email = ?, phone = ? 
         WHERE id = ?"
    )
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.email)
    .bind(&req.phone)
    .bind(contact_id)
    .execute(pool)
    .await?;

    Ok(ContactResponse {
        id: contact_id,
        first_name: req.first_name,
        last_name: req.last_name,
        email: req.email,
        phone: req.phone,
    })
}

pub async fn remove(
    pool: &MySqlPool,
    username: &str,
    contact_id: i32,
) -> AppResult<()> {
    let result = sqlx::query(
        "DELETE FROM contacts WHERE id = ? AND username = ?"
    )
    .bind(contact_id)
    .bind(username)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("contact is not found".to_string()));
    }

    Ok(())
}

pub async fn search(
    pool: &MySqlPool,
    username: &str,
    req: SearchContactRequest,
) -> AppResult<ContactSearchResponse> {
    let page = req.page.unwrap_or(1).max(1);
    let size = req.size.unwrap_or(10).min(100).max(1);
    let offset = (page - 1) * size;

    // Build WHERE clause
    let mut where_clauses = vec!["username = ?".to_string()];
    let mut query_params: Vec<String> = vec![username.to_string()];

    if let Some(name) = &req.name {
        where_clauses.push("(first_name LIKE ? OR last_name LIKE ?)".to_string());
        let like_pattern = format!("%{}%", name);
        query_params.push(like_pattern.clone());
        query_params.push(like_pattern);
    }

    if let Some(email) = &req.email {
        where_clauses.push("email LIKE ?".to_string());
        query_params.push(format!("%{}%", email));
    }

    if let Some(phone) = &req.phone {
        where_clauses.push("phone LIKE ?".to_string());
        query_params.push(format!("%{}%", phone));
    }

    let where_clause = where_clauses.join(" AND ");

    // Count total items
    let count_query = format!("SELECT COUNT(*) FROM contacts WHERE {}", where_clause);
    let mut count_query_builder = sqlx::query_as::<_, (i64,)>(&count_query);
    for param in &query_params {
        count_query_builder = count_query_builder.bind(param);
    }
    let total_item = count_query_builder.fetch_one(pool).await?.0;

    // Fetch contacts
    let select_query = format!(
        "SELECT id, first_name, last_name, email, phone, username 
         FROM contacts 
         WHERE {} 
         LIMIT ? OFFSET ?",
        where_clause
    );

    let mut query_builder = sqlx::query_as::<_, Contact>(&select_query);
    for param in &query_params {
        query_builder = query_builder.bind(param);
    }
    query_builder = query_builder.bind(size).bind(offset);

    let contacts = query_builder.fetch_all(pool).await?;

    let data: Vec<ContactResponse> = contacts.into_iter().map(|c| c.into()).collect();

    let total_page = ((total_item as f64) / (size as f64)).ceil() as i32;

    Ok(ContactSearchResponse {
        data,
        paging: PagingResponse {
            page,
            total_page,
            total_item,
        },
    })
}