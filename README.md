# Code from "Zero To Production In Rust"

https://www.zero2prod.com/

写経 (2回目)

## Database

### 3.8.4.2.1 sqlx-cli

```
cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres
```

```
export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
sqlx database create
sqlx migrate (add|run)

5.3.3 Sqlx Offline Mode
cargo sqlx prepare -- --lib
```
