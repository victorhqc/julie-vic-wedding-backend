# julie-vic-wedding-backend
Jule & Vic Wedding Backend


## Requirements

-   Rust >= 1.40.0
-   Docker (For development)

## Development

Start Postgresql

```sh
docker run -it --rm --name julie-vic -p 5432:5432 postgres
```

Run migrations

```sh
DATABASE_URL=postgresql://postgres@localhost:5432/postgres diesel migration run
```
