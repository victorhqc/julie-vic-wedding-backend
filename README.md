# julie-vic-wedding-backend
Jule & Vic Wedding Backend


## Requirements

-   Rust >= 1.40.0
-   Docker (For development)

## Development

Duplicate the `.env.example` and rename it as `.env`

Start Postgresql

```sh
docker run -it --rm --name julie-vic -p 5432:5432 postgres
```

Run migrations

```sh
diesel migration run
```

Run in development mode.

```sh
cargo run

# Or with cargo-watch for updating after changes
cargo watch -x "run"
```

For watching changes install `cargo-watch`

```sh
cargo install cargo-watch
```
