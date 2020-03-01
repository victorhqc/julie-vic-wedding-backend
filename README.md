# julie-vic-wedding-backend

Julie & Vic Wedding Backend

## Description

This project is built in separate pieces:

-   **Core:** Contains mainly DB reusable code.
-   **Api:** Web API Specific code.

## Architecture

![Data flow](./architecture.svg)

Data flow

![DB Entities](./db_entities.svg)

DB Entities

## Run in development mode

You can also run this project in the root but you'll need several terminals open for that.

1.  Start the DB

    ```sh
    make postgres
    ```

1.  Run migrations (You'll need to change location to `julie-vic-wedding-core`)

1.  Run api

    ```sh
    cargo watch -x 'run --bin=julie-vic-wedding-api'
    ```

    Or the CLI

    ```sh
    cargo watch -x 'run --bin=julie-vic-wedding-cli'
    ```

## Build production bundle

To build a release target from Windows or Mac OS, is required to have Docker to cross-compile. This because it relies on Open SSL and building it outside Linux hasn't been trivial. Many posts exists about it that describe it better.

A solution for it is use [rust-musl-builder](https://github.com/emk/rust-musl-builder) to build it.

```sh
make version=0.1.0 build
```

## Setup Service

For documentation purposes, here's how to setup the service manually.

Make sure the binary is in the server using `target/x86_64-unknown-linux-musl/release/julie-vic-wedding-api`.

Follow instructions from [here](https://medium.com/@benmorel/creating-a-linux-service-with-systemd-611b5c8b91d6)

## References

-   [Cross compiling a simple Rust Web App](https://www.andrew-thorburn.com/cross-compiling-a-simple-rust-web-app/)
-   [Cross Compiling Linux Binaries from Mac OS](http://timryan.org/2018/07/27/cross-compiling-linux-binaries-from-macos.html)
-   [rust-musl-builder](https://github.com/emk/rust-musl-builder)
