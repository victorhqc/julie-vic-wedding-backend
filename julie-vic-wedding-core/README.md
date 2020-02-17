# julie-vic-wedding-core

Julie & Vic Wedding Core

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
# When bootstraping the project (so schema file doesn't get overwritten)
diesel migration run --locked-schema

# Whenever a new migration gets added.
diesel migration run

```
