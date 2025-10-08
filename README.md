
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

**Body:**

```json
{
  "first_name": "Muhh",
  "last_name": "Dipzz",
  "email": "dipz@example.com",
  "phone": "32423423434"
}
```

### 🔸 Update Contact

```http
PUT /api/contacts/:id
```

**Same format as create.**

### 🔸 Get Contact

```http
GET /api/contacts/:id
```

### 🔸 Search Contacts

```http
GET /api/contacts?name=Dipz&email=dipz@example.com&page=1&size=10
```

### 🔸 Delete Contact

```http
DELETE /api/contacts/:id
```

---

## 🏠 Address API

All endpoints require:

**Headers:**

* `Authorization: token`

### 🔸 Create Address

```http
POST /api/contacts/:contactId/addresses
```

### 🔸 Update Address

```http
PUT /api/contacts/:contactId/addresses/:addressId
```

### 🔸 Get Address by ID

```http
GET /api/contacts/:contactId/addresses/:addressId
```

### 🔸 List All Addresses

```http
GET /api/contacts/:contactId/addresses
```

### 🔸 Delete Address

```http
DELETE /api/contacts/:contactId/addresses/:addressId
```

> ✅ Response formats follow a consistent structure with `data` on success and `errors` on failure.

---

## 👤 User API

### 🔸 Register

```http
POST /api/users
```

**Body:**

```json
{
  "username": "dipzz",
  "password": "rahasia",
  "name": "Muhhdipzz"
}
```

### 🔸 Login

```http
POST /api/users/login
```

**Response:**

```json
{
  "data": {
    "token": "unique-token"
  }
}
```

### 🔸 Update Profile

```http
PATCH /api/users/current
```

**Optional fields:**

```json
{
  "name": "Updated Name",
  "password": "newpassword"
}
```

### 🔸 Get Current User

```http
GET /api/users/current
```

### 🔸 Logout

```http
DELETE /api/users/logout
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
