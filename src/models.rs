use serde::{Deserialize, Serialize};
use validator::Validate;

// User Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub password: String,
    pub name: String,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 100))]
    pub username: String,
    #[validate(length(min = 1, max = 100))]
    pub password: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, max = 100))]
    pub username: String,
    #[validate(length(min = 1, max = 100))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub username: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// Contact Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Contact {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub username: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateContactRequest {
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(max = 100))]
    pub last_name: Option<String>,
    #[validate(email, length(max = 200))]
    pub email: Option<String>,
    #[validate(length(max = 20))]
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateContactRequest {
    #[validate(length(min = 1, max = 100))]
    pub first_name: String,
    #[validate(length(max = 100))]
    pub last_name: Option<String>,
    #[validate(email, length(max = 200))]
    pub email: Option<String>,
    #[validate(length(max = 20))]
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchContactRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub page: Option<i32>,
    pub size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContactSearchResponse {
    pub data: Vec<ContactResponse>,
    pub paging: PagingResponse,
}

// Address Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Address {
    pub id: i32,
    pub street: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: String,
    pub postal_code: String,
    pub contact_id: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAddressRequest {
    #[validate(length(max = 255))]
    pub street: Option<String>,
    #[validate(length(max = 100))]
    pub city: Option<String>,
    #[validate(length(max = 100))]
    pub province: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub country: String,
    #[validate(length(min = 1, max = 10))]
    pub postal_code: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateAddressRequest {
    #[validate(length(max = 255))]
    pub street: Option<String>,
    #[validate(length(max = 100))]
    pub city: Option<String>,
    #[validate(length(max = 100))]
    pub province: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub country: String,
    #[validate(length(min = 1, max = 10))]
    pub postal_code: String,
}

#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub id: i32,
    pub street: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: String,
    pub postal_code: String,
}

// Common Models
#[derive(Debug, Serialize)]
pub struct PagingResponse {
    pub page: i32,
    pub total_page: i32,
    pub total_item: i64,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub errors: String,
}

impl From<Contact> for ContactResponse {
    fn from(contact: Contact) -> Self {
        Self {
            id: contact.id,
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
            phone: contact.phone,
        }
    }
}

impl From<Address> for AddressResponse {
    fn from(address: Address) -> Self {
        Self {
            id: address.id,
            street: address.street,
            city: address.city,
            province: address.province,
            country: address.country,
            postal_code: address.postal_code,
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            name: user.name,
        }
    }
}