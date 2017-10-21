# Rust WebAssembly loader
<p align="center">
  <img src="https://raw.githubusercontent.com/lord/img/master/logo-wargo-loader.png" alt="wargo-loader: Rust for Webpack" width="226">
  <br>
  <a href="https://travis-ci.org/lord/wargo-loader"><img src="https://travis-ci.org/lord/wargo-loader.svg?branch=master" alt="Build Status"></a>
  <a href="https://www.npmjs.com/package/wargo-loader"><img src="https://img.shields.io/npm/v/wargo-loader.svg" alt="NPM Version"></a>
</p>

### Usage

This is a simple Webpack loader that shells out to cargo to build a Rust project targeting WebAssembly. See [this post](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627) for
more details on using Rust to target the web.

To use it, first install the package:

```bash
$ npm install wargo-loader
```

Configure the loader in your Webpack config:

```js
module.exports = {
  // ...
  module: {
    rules: [
      {
        test: /\.rs$/,
        use: {
          loader: 'wargo-loader',
          options: {
            // Path to your 'build' or 'dist' directory relative to project root
            path: 'build/',
          }
        }
      },
      // ...
    ]
  }
}
```

Note: if you are using `file-loader`, make sure to add `.wasm` to the test field, otherwise the module will not be copied! (e.g. `test: /\.(wasm|jpg|jpeg|png|gif|svg|eot|ttf|woff|woff2)$/i,`).

```js
const wasm = require('./main.rs')
wasm.initialize().then(module => {
  // Use your module here
  const doub = module.cwrap('doub', 'number', ['number'])
  console.log(doub(21))
})
```

or with async/await:

```js
async function loadAndUseWasm() {
  const wasm = require('./main.rs')
  const module = await wasm.initialize()
  const doub = module.cwrap('doub', 'number', ['number'])
  console.log(doub(21))
}
loadAndUseWasm()
```

### Configuration

The following options can be added to the Webpack loader query:

| Name | Description | Required | Default |
| ---- | ----------- | -------- | ------- |
| `release` | Whether or not to pass the `--release` flag to cargo | false | false |
| `path` | Path to your webpack output folder relative to project root | true | '' |

### Example

Check out the [example](example) directory for a simple Hello World example.

This project is based off of [rust-emscripten-loader](https://github.com/mrdziuban/rust-emscripten-loader)
by [mrdziuban](https://github.com/mrdziuban).
