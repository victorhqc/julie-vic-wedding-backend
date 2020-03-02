# julie-vic-wedding-cli

Julie & Vic Wedding CLI

## Requirements

-   Rust >= 1.40.0
-   Docker (For development)

## Development

Duplicate the `.env.example` and rename it as `.env`

_Note: Make sure you followed the instructions in the core first, as it shows how to setup DB_

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

## Commands

### Generate tokens

```sh
julie-vic-wedding-cli generate tokens --a 50
```
