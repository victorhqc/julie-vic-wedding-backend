# julie-vic-wedding-backend

Julie & Vic Wedding Backend

## Requirements

- Rust >= 1.40.0
- Docker (For development)

## Development

Duplicate the `.env.example` and rename it as `.env`

Start Postgresql

```sh
docker run -it --rm --name julie-vic -p 5432:5432 postgres
```

Run migrations

```sh
# When bootstraping the project (so schema file doesn't get overwritten)
diesel migration run --locked-schema

# Whenever a new migration gets added.
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
