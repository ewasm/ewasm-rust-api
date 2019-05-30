# ewasm-rust-api

![Build](https://circleci.com/gh/ewasm/ewasm-rust-api.svg?style=shield&circle-token=:circle-token)
![Version](https://img.shields.io/crates/v/ewasm-api.svg)

This project aims to give a low-level and a high-level binding to ewasm from Rust.

# Usage

Add the dependency, as usual:
```toml
[dependencies]
ewasm-api = "0.9"
```

In your project, include the prelude:
```rust
use ewasm_api::prelude::*;
```

Additionally there is support for some macros to make creating contracts easier:
```rust
#[macro_use]
extern crate ewasm_api;

use ewasm_api::prelude::*;

fn entry() {
    // The actual contract code goes here.
}

ewasm_entry_point!(entry);
```

Other modules are available as well, outside of the prelude. Refer to the documentation for more info.

`ewasm-rust-api` builds with various feature sets:
- `default`: Builds with `wee_alloc` as the global allocator and with the Rust standard library.
- `qimalloc`: Builds with [qimalloc](https://github.com/wasmx/qimalloc) as the global allocator.
- `experimental`: Exposes the experimental bignum system library API.

To enable specific features include the dependency as follows:
```toml
[dependencies.ewasm_api]
version = "0.9"
default-features = false
features = ["std", "qimalloc"]
```
Further documentation is available [here](https://docs.rs/ewasm_api/).

## Author(s)

Alex Beregszaszi, Jake Lang

## License

Apache 2.0
