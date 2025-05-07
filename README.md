# Synnapse Bridge - API Backend

## Project Overview

Synnapse Bridge is a robust, Rust-based API backend system designed to serve as the data management layer for the Synnapse application.

This service provides a RESTful API for managing users (persons), their associated data entries, and a comprehensive permissions system.
Built with Rust, Rocket, and Diesel, it offers high performance, strong type safety, and a well-organized architecture.

### Key Features

- Complete RESTful API with CRUD operations for all resources
- Secure password handling with hashing and salting
- OpenAPI/Swagger documentation for easy API exploration
- Database migrations for version control of your schema
- Clear separation of concerns across three crates (API, Database, Application)

---

### Technology Stack

- **Language**: Rust
- **Web Framework**: Rocket
- **ORM**: Diesel
- **Database**: Currently SQLite, with plans for PostgreSQL
- **Documentation**: OpenAPI/Swagger

---

## Project Structure

The project is organized into three distinct crates, each with a specific responsibility:

### 1. API Crate (`/api`)

- **Purpose**: Handles HTTP endpoints, routing, and API documentation
- **Framework**: Rocket for HTTP server functionality
- **Key Components**:
  - [`src/lib.rs`](api/src/lib.rs) - Main API configuration and server setup
  - [`src/models.rs`](api/src/models.rs) - API-specific data structures
  - [`src/routes/`](api/src/routes/) - Organized endpoint definitions:
    - [`entries.rs`](api/src/routes/entries.rs) - Entry resource endpoints
    - [`permissions.rs`](api/src/routes/permissions.rs) - Permission resource endpoints
    - [`person.rs`](api/src/routes/person.rs) - Person resource endpoints

### 2. Database Crate (`/db`)

- **Purpose**: Handles database operations, models, and schema management
- **Framework**: Diesel ORM for database interactions
- **Key Components**:
  - [`src/lib.rs`](db/src/lib.rs) - Database core functionality
  - [`src/models.rs`](db/src/models.rs) - Database entity definitions
  - [`src/schema.rs`](db/src/schema.rs) - Auto-generated schema from migrations
  - [`src/interactions.rs`](db/src/interactions.rs) - CRUD operations implementation
  - [`src/crypto.rs`](db/src/crypto.rs) - Security utilities for password handling
  - [`migrations/`](db/migrations/) - Versioned database schema changes:
    - Tables for persons, entries, and permissions

### 3. Main Application (`/src`)

- **Purpose**: Entry point and configuration for the entire application
- **Key Component**:
  - [`main.rs`](src/main.rs) - Application bootstrapping and configuration

### Additional Project Files

- [`Cargo.toml`](Cargo.toml) - Rust package configuration
- [`diesel.toml`](diesel.toml) - Diesel ORM configuration
- [`flake.nix`](flake.nix) - Nix package manager configuration
- `test.db` - SQLite database for development

---

### TODOs

- [x] CRUD operations in `db/src/interactions.rs`.
  - [x] Create operations
  - [x] Read operations
  - [x] Update operations
  - [x] Delete operations

- Implement RESTful API endpoints in [api/src/routes.rs](api/src/routes.rs)
  See [API Routes](#api-routes) section for details.

- [x] Generate API documentation using Swagger/OpenAPI
- [ ] Develop comprehensive test suite for API endpoints
- [ ] Configure and test with PostgreSQL database
- [ ] Containerize application with Docker
- [ ] Finalize password management strategy (currently using [db/src/crypto.rs](db/src/crypto.rs) for password hashing and salting)
- [ ] Implement JWT-based authentication system

### API Routes

Currently implemented API routes:

- [x] GET `/api/person` - Get all persons
- [x] GET `/api/person/<person_id>` - Get a single person by ID
- [x] POST `/api/person` - Create a new person
- [x] PUT `/api/person/<person_id>` - Update an existing person
- [x] DELETE `/api/person/<person_id>` - Delete a person

- [x] GET `/api/entries` - Get all entries
- [x] GET `/api/entries/<entry_id>` - Get a single entry by ID
- [x] GET `/api/entries/by-person/<person_id>` - Get entries by person ID
- [x] POST `/api/entries` - Create a new entry
- [x] PUT `/api/entries/<entry_id>` - Update an existing entry
- [x] DELETE `/api/entries/<entry_id>` - Delete an entry

- [x] GET `/api/permissions` - Get all permissions
- [x] GET `/api/permissions/<permission_id>` - Get a single permission by ID
- [x] GET `/api/permissions/by-person/<person_id>` - Get permissions by person ID
- [x] POST `/api/permissions` - Create a new permission
- [x] PUT `/api/permissions/<permission_id>` - Update an existing permission
- [x] DELETE `/api/permissions/<permission_id>` - Delete a permission

We may need routes for fetching the history of things and we may need to limit the number of entries returned and do pages in the frontend.
Also /health endpoint with information about database status and other things for the admin dashboard.
