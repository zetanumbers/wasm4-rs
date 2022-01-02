# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

This is the first working release of `wasm4` crate! You can add this crate
as a dependency to your cartridge's code by specifying it in Cargo.toml:

```toml
[dependencies]
wasm4 = "0.0.3" # add this line
```

### Fully safe statefull cartridges

You can now create fully safe statefull cartridges like this:

```rust
// src/main.rs
#![no_main]

struct MyRuntime {
    count: i32,
}

// prints "tick..." every second
impl wasm4::Runtime for MyRuntime {
    fn start(_: wasm4::Resources) -> Self {
        MyRuntime { count: 0 }
    }

    fn update(&mut self) {
        if self.count % 60 == 0 {
            wasm4::trace("tick");
            self.count = 0;
        }
        self.count += 1;
    }
}

wasm4::main! { MyRuntime }
```

Notice that this is not a library (`src/lib.rs`), but an executable (`src/main.rs`). This is done for better integration with cargo tooling, being able to run these cartriges using `cargo run`.

### Documentation

 - <csr-id-7b490feace43670f3f2100595ab6f0a1ee988d62/> add changelog for `wasm4`

### New Features

 - <csr-id-7a1d0114338f2f9c33580731ada2d348b9a5abbc/> add runtime creation functionality

### refactor (BREAKING)

 - <csr-id-233c4f95d0e7af696a6f19313257d579d4ce45f3/> change to the new experimental api

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release over the course of 18 calendar days.
 - 5 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - bump crates versions ([`2c68e02`](https://github.com/ZetaNumbers/wasm4-rs/commit/2c68e023407205b7bb4f10a8111e9e78e368bbab))
    - add runtime creation functionality ([`7a1d011`](https://github.com/ZetaNumbers/wasm4-rs/commit/7a1d0114338f2f9c33580731ada2d348b9a5abbc))
    - `sound::Mode` values are bit aligned for `sound::Flags` ([`8027ce0`](https://github.com/ZetaNumbers/wasm4-rs/commit/8027ce001a85408fe16fd54aae3516d02727377e))
    - change to the new experimental api ([`233c4f9`](https://github.com/ZetaNumbers/wasm4-rs/commit/233c4f95d0e7af696a6f19313257d579d4ce45f3))
    - add changelog for `wasm4` ([`7b490fe`](https://github.com/ZetaNumbers/wasm4-rs/commit/7b490feace43670f3f2100595ab6f0a1ee988d62))
    - Release 0.0.2 ([`d7fbc7c`](https://github.com/ZetaNumbers/wasm4-rs/commit/d7fbc7ca18d6badbc338c6df23aa344593822cf0))
    - Release 0.0.1 ([`0090ea9`](https://github.com/ZetaNumbers/wasm4-rs/commit/0090ea907b415a9a7e1034926ec6ac24c10ab938))
    - Prepare manifest for publish ([`3b31a1e`](https://github.com/ZetaNumbers/wasm4-rs/commit/3b31a1ed6f3a3f1b6f00b6f2539b8ca8a2ea3a3a))
    - Fix docs a bit ([`cbe67e0`](https://github.com/ZetaNumbers/wasm4-rs/commit/cbe67e0fccc5b16635930765e17fba0f7f06a5d4))
    - Implement sys bindings; Implement sound module ([`babbc6d`](https://github.com/ZetaNumbers/wasm4-rs/commit/babbc6dd6a0aa4e438dd490d639f98f2add2f9d8))
</details>

## 0.0.2 (2021-12-16)

 - Initial release

