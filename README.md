# React  + Rocket

This is a template repository that provides a starting point for developing a web application with a JS front end and a Rust back end.
It requires you to have the [Rust nightly compiler](https://www.rust-lang.org/tools/install) installed.

## To use

Press the "use this template" button above and then complete the following form.

Clone the resulting repository, call `yarn install`, and off you go!

### How it works

**Note: all directories below are relative to the project root**

First, Webpack and Babel compile the front end sources in `src/` which are output to `dist/`.
Then we compile and run the Rust server found in `src/`. 
The server is configured to serve the static files output by Webpack using a special route.
This route will resolve the requested URI inside of the `dist/public` directory.
For example, if the client requests `example.com/public/js/main.js`, the server looks for `dist/public/js/main.js`.

The front end client is a single page application in [React](reactjs.org) with `App.js` as its entry point.
It uses ReactDOM to render inside the `<App />` element found in `public/index.html`.
There is no directory structure inside of `src/` so that you are free to create one that suits your needs.

The server uses a framework called [Rocket](rocket.rs) for routing.
There are a couple of routes already set up: one for the index route and another to serve static files requested by the front end client.
Webpack and Cargo both use a similar (and co-existable) directory structure, i.e. they both use the `src/` directory, but Cargo outputs to `target/`.

### Building and Running the server

You can build and run each component (front/back) and serve using yarn commands:

```sh
yarn build   # Webpack + Babel (Frontend)
yarn compile # cargo build (Backend)
yarn serve   # yarn build + cargo run (Both)
```

## Future Features and Todos

* WASM and WASI! This is really the impetus behind this effort.
* Default tests and/or test skeleton?
* Defaults for 404 etc.
