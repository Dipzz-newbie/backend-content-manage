
---

# 🦀 Backend Content Management API

A fast, reliable, and modular **RESTful API** built with **Rust** for managing content-driven backend systems. Designed for **scalability**, **performance**, and **real-world scenarios**, leveraging Rust’s safety and speed.

---

## 📦 Features

* ⚙️ RESTful API architecture
* 🧱 Modular code structure (`src/`, `migrations/`, etc.)
* 🛢️ Database support with migration handling
* 📄 JSON-based request/response
* 🔐 Token-based authentication
* 🚀 Built for learning, extensibility, and real-world backend patterns

---

## 🛠️ Tech Stack

| Tool / Library          | Description               |
| ----------------------- | ------------------------- |
| **Rust**                | Systems-level programming |
| **Actix-web** (assumed) | Web framework for Rust    |
| **Diesel / SQLx**       | ORM and database access   |
| **PostgreSQL**          | Relational database       |
| **dotenv**              | Environment config        |

> ℹ️ **Note:** Please update actual crates used in `Cargo.toml` if different from assumptions above.

---

## 📁 Project Structure

```
.
├── migrations/        # Database schema & migrations
├── src/               # Main API logic (routes, models, services)
├── .env               # Environment configuration
├── .gitignore
└── Cargo.toml         # Rust project configuration
```

---

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/Dipzz-newbie/backend-content-manage.git
cd backend-content-manage
```

### 2. Setup Environment Variables

Create a `.env` file in the root directory:

```env
DATABASE_URL=postgres://user:password@localhost/db_name
PORT=8080
```

### 3. Run Migrations

```bash
diesel migration run  # or use sqlx / refinery depending on setup
```

### 4. Build and Run the Server

```bash
cargo run
```

The API will be available at:

```
http://localhost:8080
```

---

## 📬 API Endpoints Overview

### 🔹 Posts

| Method | Endpoint      | Description         |
| ------ | ------------- | ------------------- |
| GET    | `/posts`      | Get all posts       |
| POST   | `/posts`      | Create a new post   |
| PUT    | `/posts/{id}` | Update a post by ID |
| DELETE | `/posts/{id}` | Delete a post by ID |

---

## 📇 Contact API

### 🔸 Create Contact


```http
POST /api/contacts
```

**Headers:**

* `Authorization: token`

Request Body :

```json
{
  "first_name" : "Dipzz",
  "last_name" : "Muhh",
  "email" : "dipzz@example.com",
  "phone" : "32423423434"
}
```

Response Body Success :

```json
{
  "data" : {
    "id" : 1,
    "first_name" : "Dipzz",
    "last_name" : "Muhh",
    "email" : "dipz@example.com",
    "phone" : "32423423434"
  }
}
```

Response Body Error :

```json
{
  "errors" : "Email is not valid format"
}
```

### 🔸 Update Contact

```http
PUT /api/contacts/:id
```

**Headers:**

* `Authorization: token`

Request Body :

```json
{
  "first_name" : "Muhh",
  "last_name" : "Dipzz",
  "email" : "dipzz@pzn.com",
  "phone" : "32423423434"
}
```

Response Body Success :

```json
{
  "data" : {
    "id" : 1,
    "first_name" : "Muhh",
    "last_name" : "Dipzz",
    "email" : "dipzz@example.com",
    "phone" : "32423423434"
  }
}
```

Response Body Error :

```json
{
  "errors" : "Email is not valid format"
}
```

**Same format as create.**

### 🔸 Get Contact

```http
GET /api/contacts/:id
```

**Headers:**

* `Authorization: token`

Response Body Success :

```json
{
  "data" : {
    "id" : 1,
    "first_name" : "Dipzz",
    "last_name" : "Muhh",
    "email" : "dipzz@example.com",
    "phone" : "32423423434"
  }
}
```

Response Body Error :

```json
{
  "errors" : "Contact is not found"
}
```



### 🔸 Search Contacts

```http
GET /api/contacts
```

**Headers:**

* `Authorization: token`

Query params: 
- name : Search by first_name or last_name, using like, **optional** 
- email : Search by email using like, **optional** 
- phone : Search by phone using like, **optional** 
- page : number of page, default 1 
- size : size per page, default 10 Response Body Success 

Response Body Success :

```json
{
  "data" : [
    {
      "id" : 1,
      "first_name" : "Muhh",
      "last_name" : "Dipzz",
      "email" : "dipzz@example.com",
      "phone" : "32423423434"
    },
    {
      "id" : 2,
      "first_name" : "Muhh",
      "last_name" : "Dipzz",
      "email" : "Dipzz@example.com",
      "phone" : "32423423434"
    }
  ],
  "paging" : {
    "page" : 1,
    "total_page" : 3,
    "total_item" : 30
  }
}
```

Response Body Error :

```json
{
  "errors" : "Email is not valid format"
}
```

### 🔸 Delete Contact

```http
DELETE /api/contacts/:id
```

**Headers:**

* `Authorization: token`

Response Body Success :

```json
{
  "data" : "OK"
}
```

Response Body Error :

```json
{
  "errors" : "Contact is not found"
}
```

---

## 🏠 Address API

All endpoints require:


### 🔸 Create Address

```http
POST /api/contacts/:contactId/addresses
```

**Headers:**

* `Authorization: token`

Request Body :

```json
{
  "street" : "Jalan apa",
  "city" : "Kota apa",
  "province" : "Provinsi apa",
  "country" : "Negara apa",
  "postal_code" : "Kode pos"
}
```

Response Body Success :

```json
{
  "data" : {
    "id" : 1,
    "street" : "Jalan apa",
    "city" : "Kota apa",
    "province" : "Provinsi apa",
    "country" : "Negara apa",
    "postal_code" : "Kode pos"
  }
}
```

Response Body Error :

```json
{
  "errors" : "Country is required" 
}
```

### 🔸 Update Address

```http
PUT /api/contacts/:contactId/addresses/:addressId
```

**Headers:**

* `Authorization: token`

Request Body :
```json
{
  "street" : "Jalan apa",
  "city" : "Kota apa",
  "province" : "Provinsi apa",
  "country" : "Negara apa",
  "postal_code" : "Kode pos"
}
```

Response Body Success :

```json
{
  "data" : {
    "id" : 1,
    "street" : "Jalan apa",
    "city" : "Kota apa",
    "province" : "Provinsi apa",
    "country" : "Negara apa",
    "postal_code" : "Kode pos"
  }
}
```

Response Body Error :

```json
{
  "errors" : "Country is required"
}
```

### 🔸 Get Address by ID

```http
GET /api/contacts/:contactId/addresses/:addressId
```

**Headers:**

* `Authorization: token`

Response Body Success :

```json
{
  "data" : {
    "id" : 1,
    "street" : "Jalan apa",
    "city" : "Kota apa",
    "province" : "Provinsi apa",
    "country" : "Negara apa",
    "postal_code" : "Kode pos"
  }
}
```

Response Body Error :

```json
{
  "errors" : "contact is not found"
}
```

### 🔸 List All Addresses

```http
GET /api/contacts/:contactId/addresses
```

**Headers:**

* `Authorization: token`

Response Body Success :

```json 
{
  "data" : [
    {
      "id" : 1,
      "street" : "Jalan apa",
      "city" : "Kota apa",
      "province" : "Provinsi apa",
      "country" : "Negara apa",
      "postal_code" : "Kode pos"
    },
    {
      "id" : 1,
      "street" : "Jalan apa",
      "city" : "Kota apa",
      "province" : "Provinsi apa",
      "country" : "Negara apa",
      "postal_code" : "Kode pos"
    }
  ]
}
```

Response Body Error :

```json
{
  "errors" : "contact is not found"
}
```

### 🔸 Delete Address

```http
DELETE /api/contacts/:contactId/addresses/:addressId
```

**Headers:**

* `Authorization: token`

Response Body Success :

```json
{
  "data" : "OK"
}
```

Response Body Error :

```json
{
  "errors" : "address is not found"
}
```

> ✅ Response formats follow a consistent structure with `data` on success and `errors` on failure.

---

## 👤 User API

### 🔸 Register

```http
POST /api/users
```

Request Body :

```json
{
  "username": "dipzz",
  "password": "rahasia",
  "name": "Muhhdipzz"
}
```

Response Body Success :

```json
{
  "data" : {
    "username" : "MuhhDipzz",
    "name" : "Dipzz"
  }
}
```

Response Body Error : 

```json
{
  "errors" : "Username already registered"
}
```

### 🔸 Login

```http
POST /api/users/login
```

Request Body :

```json
{
  "username" : "dipzz",
  "password" : "rahasia"
}
```

Response Body Success : 

```json
{
  "data" : {
    "token" : "unique-token"
  }
}
```

Response Body Error :

```json
{
  "errors" : "Username or password wrong"
}
```

### 🔸 Update Profile

```http
PATCH /api/users/current
```

**Headers:**

* `Authorization: token`

Request Body :

```json
{
  "name" : "newName", // optional
  "password" : "new password" // optional
}
```

Response Body Success : 

```json
{
  "data" : {
    "username" : "Dipzz",
    "name" : "newName"
  }
}
```

Response Body Error : 

```json
{
  "errors" : "Name length max 100"
}
```

### 🔸 Get Current User

```http
GET /api/users/current
```

**Headers:**

* `Authorization: token`

Response Body Success:

```json
{
  "data" : {
    "username" : "MuhhDipzz",
    "name" : "Dipzz"
  }
}
```

Response Body Error : 

```json
{
  "errors" : "Unauthorized"
}
```

### 🔸 Logout

```http
DELETE /api/users/logout
```

**Headers:**

* `Authorization: token`

Response Body Success : 

```json
{
  "data" : "OK"
}
```

Response Body Error : 

```json
{
  "errors" : "Unauthorized"
}
```

---

## 🧪 Testing

To run unit and integration tests:

```bash
cargo test
```

---

## 🙌 Contributing

We welcome contributions!

1. Fork the repository
2. Create a new branch: `git checkout -b feature/my-feature`
3. Commit your changes: `git commit -m "Add my feature"`
4. Push your branch: `git push origin feature/my-feature`
5. Open a Pull Request

---

## 📄 License

Licensed under the [MIT License](LICENSE).

---

## 🙏 Acknowledgements

Built as a backend learning project using **Rust**, **Actix**, and **PostgreSQL**. Contributions and feedback are welcome!

---

Let me know if you want this turned into a `README.md` file or need badges (e.g., GitHub Actions, Rust version, etc.) added to the top!