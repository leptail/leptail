# Leptail Demo
This is demo project for leptail component library. The project is SSR version of [Leptos](https://github.com/leptos-rs/leptos)
with [Axum](https://github.com/tokio-rs/axum) as server


## How to build
Install the following cargo tools:
```bash 
cargo install cargo-leptos just stylance-cli
```

The open-props local installation requries npm package to be installed. Follow the stepls
1. install npm or pnpm or your favorite build node package manager
2. Run the install command ex: `pnpm install`


Now, you can use just ot run in dev mode or prod, or build it for production. Check justfile for information. 
To run in dev mode, 
```bash 
just dev 
```
Note: `crtl + c` on dev recipe doesn't kill sytlance process. Manullay kill using `killall stylance` once in a while.  

To run in production mode, run 
```bash 
just prod 
```

For more detail on how to build or deploy axum project, checkout [Leptos Axum Workspace Project](https://github.com/leptos-rs/start-axum-workspace)

