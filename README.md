# Leptail

*Component Library for [Leptos](https://github.com/leptos-rs/leptos)*: Component Library that aims to be headless, themeable using [tailwindcss](https://github.com/tailwindlabs/tailwindcss). While being flexible to how developer/designer willing to theme, the library aims to provide off the shelf themes so that one can get started quickly. Read more at [Leptail Readme](https://github.com/leptail/leptail/leptail).  

This project is started with [Leptos Axum Workspace](https://github.com/leptos-rs/start-axum-workspace) template using tool [cargo-leptos](https://github.com/akesson/cargo-leptos) with [Axum](https://github.com/tokio-rs/axum) as server for the demo app.


## Note
This project is still in the experimental level. Only one component is written and complete tooling is still being worked out.

## Credits
Special thanks to [Leptail](https://github.com/lpotthast/leptonic) project(a component library for Leptos) through which some of the code example is copied from.

## How to build

Quick command to build and run

```bash
cargo leptos watch
```

For more detail on how to build or deploy axum project, checkout [Leptos Axum Workspace Project](https://github.com/leptos-rs/start-axum-workspace)



## Project Organisation 
The project is divided into sub projects

### leptail
The core project that provides headless reactive component.

### tailwind-utils
The helper libraries that defines types and utitliy function for tailwindcss, which is  

### theme-moonlight 

### theme-gradiance 
