Create a database.
Update the DATABASE_URL in the .env file with your database connection details.

1. Install sqlx

```bash
cargo install sqlx-cli
```

2. Create the database 

```bash
sqlx database create
```

3. Add migration with revert

```bash
sqlx migrate add -r create_schema
```

4. Run migrations

```bash
sqlx migrate run
```