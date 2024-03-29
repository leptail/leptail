# Leptail

*Component Library for [Leptos](https://github.com/leptos-rs/leptos)*: Component Library that aims to be headless, themeable using [tailwindcss](https://github.com/tailwindlabs/tailwindcss). While being flexible to how developer/designer willing to theme, the library aims to provide off the shelf themes so that one can get started quickly. Read more at [Leptail Core Readme](https://github.com/leptail/leptail/tree/main/leptail).  

This project is started with [Leptos Axum Workspace](https://github.com/leptos-rs/start-axum-workspace) template using tool [cargo-leptos](https://github.com/akesson/cargo-leptos) with [Axum](https://github.com/tokio-rs/axum) as server for the demo app.


## Note
The project is still in the experimental level. Complete tooling is still being worked out.


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

## Project Organisation 
The project is divided into sub projects

### leptail
The core project that provides headless reactive component.

### tailwind-utils
The helper libraries that defines types and utitliy function for tailwindcss, which is  

### theme-moonlight 
A light and dark theme that is easy on eyes  

### theme-gradiance 
A light and dark theme with colourful gradiance backgrounds and more.

### app 
The demo app for leptail components. 

### server 
The server part which servces the demo app. 


## Credits
Special thanks to [Leptonic](https://github.com/lpotthast/leptonic) project(a component library for Leptos) through which some of the code example is copied from.
