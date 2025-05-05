# API Backend

3 crates:

- `api`: The main API crate that contains the server and the API endpoints (rocket)

- `db`: The database crate that contains the database models and the database connection (diesel).
  - models defined in `db/src/models.rs`
  - migrations created using diesel CLI `diesel migration generate <migration_name>`
  - run migrations using `diesel migration run` THis generates the `db/src/schema.rs` file

- `self`: The entrypoint crate that contains the main function and the configuration
