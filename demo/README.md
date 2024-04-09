# Leptail Demo

This project is started with [Leptos Axum Workspace](https://github.com/leptos-rs/start-axum-workspace) template using tool [cargo-leptos](https://github.com/akesson/cargo-leptos) with [Axum](https://github.com/tokio-rs/axum) as server for the demo app.

## How to build
Quick command to build and run

```bash
cargo leptos watch
```

For more detail on how to build or deploy axum project, checkout [Leptos Axum Workspace Project](https://github.com/leptos-rs/start-axum-workspace)

If you would like to build and run in release mode, 
```bash 
cargo leptos build --release 
# then run the server form the project root directory. 
./target/server/release/server
```

