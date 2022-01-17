WASM4-rs 
====

Idiomatic bindings for WASM-4 in Rust.

[**WASM-4**](https://github.com/aduros/wasm4) is a low-level fantasy game console for building small games with WebAssembly. Game
cartridges (ROMs) are small, self-contained `.wasm` files that can be built with any programming
language that compiles to WebAssembly.

Example
----

```rust
#![no_main]

use wasm4 as w4;

struct MyRuntime {
    count: i32,
}

// prints "tick" every second
impl w4::rt::Runtime for MyRuntime {
    fn start(_: w4::rt::Resources) -> Self {
        MyRuntime { count: 0 }
    }

    fn update(&mut self) {
        if self.count % 60 == 0 {
            w4::trace("tick");
            self.count = 0;
        }
        self.count += 1;
    }
}

w4::main! { MyRuntime }
```
