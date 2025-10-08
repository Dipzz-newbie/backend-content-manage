---

# 🦀 Backend Content Management API

A fast, reliable, and modular RESTful API built with **Rust** for managing content-driven backend systems. Designed for scalability and performance, leveraging Rust’s safety and speed.

---

## 📦 Features

* ⚙️ **RESTful API** architecture
* 🧱 Modular code structure (`src/`, `migrations/`, etc.)
* 🛢️ Database support with migration handling
* 📄 JSON-based request/response
* 🚀 Built for learning, extensibility, and real-world backend scenarios

---

## 🛠️ Tech Stack

| Tool / Library                | Purpose                     |
| ----------------------------- | --------------------------- |
| **Rust**                      | Main programming language   |
| **Actix-web** *(assumed)*     | Web framework               |
| **Diesel / SQLx** *(assumed)* | ORM / DB interaction        |
| **PostgreSQL** *(or other)*   | Relational database backend |

> ⚠️ Replace the above stack with the actual crates used in your `Cargo.toml`.

---

## 📁 Project Structure

```bash
.
├── migrations/        # DB schema migrations
├── src/               # API logic (routes, models, controllers)
├── .gitignore
└── Cargo.toml         # Rust package configuration
```

---

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/Dipzz-newbie/backend-content-manage.git
cd backend-content-manage
```

### 2. Configure Environment

Create a `.env` file and configure your database and application settings.

```env
DATABASE_URL=postgres://user:password@localhost/db_name
PORT=8080
```

### 3. Run Migrations

```bash
diesel migration run  # or the tool you're using
```

### 4. Build and Run the Server

```bash
cargo run
```

API will be available at: `http://localhost:8080`

---

## 📨 Example API Endpoints

| Method | Endpoint      | Description          |
| ------ | ------------- | -------------------- |
| GET    | `/posts`      | Get all posts        |
| POST   | `/posts`      | Create new post      |
| PUT    | `/posts/{id}` | Update existing post |
| DELETE | `/posts/{id}` | Delete post by ID    |

## FUll DOCS !!!

# Address API Spec

## Create Address API

Endpoint : POST /api/contacts/:contactId/addresses

Headers :
- Authorization : token

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

## Update Address API

Endpoint : PUT /api/contacts/:contactId/addresses/:addressId

Headers :
- Authorization : token

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

## Get Address API

Endpoint : GET /api/contacts/:contactId/addresses/:addressId

Headers :
- Authorization : token

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

## List Addresses API

Endpoint : GET /api/contacts/:contactId/addresses

Headers :
- Authorization : token

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

## Remove Address API

Endpoint : DELETE /api/contacts/:contactId/addresses/:addressId

Headers :
- Authorization : token

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

---

## 🧪 Testing

Run unit and integration tests:

```bash
cargo test
```

# Contact API Spec

## Create Contact API

Endpoint : POST /api/contacts

Headers : 
- Authorization : token

Request Body :

```json
{
  "first_name" : "Eko",
  "last_name" : "Khannedy",
  "email" : "eko@pzn.com",
  "phone" : "32423423434"
}
```

Response Body Success : 

```json
{
  "data" : {
    "id" : 1,
    "first_name" : "Eko",
    "last_name" : "Khannedy",
    "email" : "eko@pzn.com",
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

## Update Contact API

Endpoint : PUT /api/contacts/:id

Headers :
- Authorization : token

Request Body :

```json
{
  "first_name" : "Eko",
  "last_name" : "Khannedy",
  "email" : "eko@pzn.com",
  "phone" : "32423423434"
}
```

Response Body Success :

```json
{
  "data" : {
    "id" : 1,
    "first_name" : "Eko",
    "last_name" : "Khannedy",
    "email" : "eko@pzn.com",
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

## Get Contact API

Endpoint : GET /api/contacts/:id

Headers :
- Authorization : token

Response Body Success :

```json
{
  "data" : {
    "id" : 1,
    "first_name" : "Eko",
    "last_name" : "Khannedy",
    "email" : "eko@pzn.com",
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

## Search Contact API

Endpoint : GET /api/contacts

Headers :
- Authorization : token

Query params :
- name : Search by first_name or last_name, using like, optional
- email : Search by email using like, optional
- phone : Search by phone using like, optional
- page : number of page, default 1
- size : size per page, default 10

Response Body Success :

```json
{
  "data" : [
    {
      "id" : 1,
      "first_name" : "Eko",
      "last_name" : "Khannedy",
      "email" : "eko@pzn.com",
      "phone" : "32423423434"
    },
    {
      "id" : 2,
      "first_name" : "Eko",
      "last_name" : "Khannedy",
      "email" : "eko@pzn.com",
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

## Remove Contact API

Endpoint : DELETE /api/contacts/:id

Headers :
- Authorization : token

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

# User API Spec

## Register User API

Endpoint :  POST /api/users 

Request Body :

```json
{
  "username" : "pzn",
  "password" : "rahasia",
  "name" : "Programmer Zaman Now"
}
```

Response Body Success :

```json
{
  "data" : {
    "username" : "pzn",
    "name" : "Programmer Zaman Now"
  }
}
```

Response Body Error : 

```json
{
  "errors" : "Username already registered"
}
```

## Login User API

Endpoint : POST /api/users/login

Request Body :

```json
{
  "username" : "pzn",
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

## Update User API

Endpoint : PATCH /api/users/current

Headers :
- Authorization : token 

Request Body :

```json
{
  "name" : "Programmer Zaman Now Lagi", // optional
  "password" : "new password" // optional
}
```

Response Body Success : 

```json
{
  "data" : {
    "username" : "pzn",
    "name" : "Programmer Zaman Now Lagi"
  }
}
```

Response Body Error : 

```json
{
  "errors" : "Name length max 100"
}
```

## Get User API

Endpoint : GET /api/users/current

Headers :
- Authorization : token

Response Body Success:

```json
{
  "data" : {
    "username" : "pzn",
    "name" : "Programmer Zaman Now"
  }
}
```

Response Body Error : 

```json
{
  "errors" : "Unauthorized"
}
```

## Logout User API

Endpoint : DELETE /api/users/logout

Headers :
- Authorization : token

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

## 👨‍💻 Contributing

1. Fork this repo
2. Create your feature branch (`git checkout -b feature/awesome`)
3. Commit your changes (`git commit -m 'Add awesome feature'`)
4. Push to the branch (`git push origin feature/awesome`)
5. Open a Pull Request

---

## 📄 License

This project is open-source and available under the [MIT License](LICENSE).

---

## 🙌 Acknowledgements

Built as a learning project using Rust and backend principles. Feel free to use, modify, and contribute.

---