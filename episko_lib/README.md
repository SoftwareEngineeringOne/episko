# Sqlx

Sqlx migrations should be handled using the sqlx cli tool.

```sh
# Create database based on the $DATABASE_URL env variable
sqlx database create

# Run migrations in migrations/
sqlx migrate run
```
