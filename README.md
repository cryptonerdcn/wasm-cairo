<div align="center">

  <h1><code>WASM-Cairo</code></h1>

  <strong>A suite of development tools and an environment for Cairo 1.10, all based on WebAssembly.</strong>

  <sub>Built with 🦀🕸 by <a href="https://twitter.com/cryptonerdcn">cryptonerdcn from Starknet Astro</a></sub>
</div>


## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build WASM-bindgen's WASM-Cairo Toolkit 
With Modules

```
wasm-pack build --release --target --out-dir ./pkg
```

No Modules

```
wasm-pack build --release --target no-modules --out-dir ./pkg
```


### 🛠️ Build Astro Editor

```
wasm-pack build --release --target no-modules --out-dir ./dist/pkg --out-name wasm-cairo
```

Then run 
```
node app.js
```
For local web instance.

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.