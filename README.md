# Rust WebAssembly loader

[![npm](https://img.shields.io/npm/v/rust-wasm-loader.svg)](https://www.npmjs.com/package/rust-wasm-loader)

### Usage

This is a simple Webpack loader that shells out to cargo to build a Rust project targeting WebAssembly. See [this post](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627) for
more details on using Rust to target the web.

To use it, first install the package:

```bash
$ npm install rust-wasm-loader
```

then configure the loader in your Webpack config:

```js
module.exports = {
  // ...
  module: {
    rules: [
      { test: /\.rs$/, loader: 'rust-wasm-loader' },
      // ...
    ]
  }
}
```

Make sure you have the `cargo`, `rustc`, and `emsdk` binaries somewhere in your `PATH`.

### Configuration

The following options can be added to the Webpack loader query:

| Name | Description | Required | Default |
| ---- | ----------- | -------- | ------- |
| `release` | Whether or not to pass the `--release` flag to cargo | false | false |

### Example

Check out the [example](example) directory for a simple Hello World example.
