## Database

### 3.8.4.2.1 sqlx-cli

```
cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres
```

```
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx database create
sqlx migrate (add|run)
```
