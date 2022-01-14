# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Framebuffer support

Now you are able to `include_spire!` from any image you have! Here's a small demo: 

```rust
#![no_main]

use wasm4 as w4;

struct SansRuntime {
    framebuffer: w4::draw::Framebuffer,
}

// displays sans
impl w4::rt::Runtime for SansRuntime {
    fn start(res: w4::rt::Resources) -> Self {
        SansRuntime {
            framebuffer: res.framebuffer,
        }
    }

    fn update(&mut self) {
        w4::include_sprites! {
            // 0xa64902 is a background color (indexed 0)
            // 0x000000 is a foreground color (indexed 1)
            // every other color is added if needed
            const PALETTE: _ = common_palette!(0xa64902, 0x000000);
            // every image may contain at most 4 colors each (transparent included)
            // all images may contain at most 4 color total (transparent excluded)
            const SMILE: _ = include_sprite!("src/sans.png");
            const TALK: _ = include_sprite!("src/talk.png");
        };

        self.framebuffer.replace_palette(PALETTE);
        self.framebuffer.blit(&SMILE, [68, 32], <_>::default());
        self.framebuffer.blit(&TALK, [0, 92], <_>::default());
    }
}

w4::main! { SansRuntime }
```

And here's the result 💀:

![hey there kiddo](https://raw.githubusercontent.com/ZetaNumbers/wasm4-rs/00e582199ed13e59153b808126e4a5ab74267a31/examples/sans/preview.png "sans")

Other framebuffer functionality also is added (`oval`, `text` and others).

### Documentation

 - <csr-id-c55457c26fa610b0556fb08a9f7e4ca02c96059f/> embed preview image
 - <csr-id-e2a63457da10b8083652272a60e4bb7d59250efe/> fix markdown headers

### New Features

 - <csr-id-06dc6afedf6ea051c5927fd06f0b7fd84a6bb55b/> add `include_sprites!` macro
 - <csr-id-883fd55ae101d7da61c8a0dd163eae3494bc8463/> add `blit` support
 - <csr-id-b27db4694664ff448172d40ebc4cdd5683017d29/> add `Framebuffer::as_cells`

### Bug Fixes

 - <csr-id-bc8c6cc9fccfabda778d080255d698eaeb97809b/> add `bitflags` crate
 - <csr-id-af5de1cb924462858946620cc41902e289a96e68/> `SpriteView` lifetime

### New Features (BREAKING)

 - <csr-id-03dd8b1d3bd064dae96b11fe9541ade2127c27eb/> Add basic framebuffer support

### Bug Fixes (BREAKING)

 - <csr-id-7aeec9545d97bddf7a56d7ed8896c28ac64c8a42/> framebuffer's byte length

### refactor (BREAKING)

 - <csr-id-d2fcdeb73c64d1877017d438d4d1aeaebea41c9f/> naming
 - <csr-id-4340269a340c2da865b0f7c143fbf0306d0ab8ae/> remove prelude
 - <csr-id-c993b2bad68b8a9b5c68d45c9ca000504dc04014/> update prelude
 - <csr-id-f8c3bd886712deb15f77733bbb55b92ed14d09d6/> privatize utils module
 - <csr-id-fe8a08a0a307e194a7bbfdd2ba94a6855bf13032/> `Framebuffer`'s methods

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 18 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - embed preview image ([`c55457c`](https://github.com/ZetaNumbers/wasm4-rs/commit/c55457c26fa610b0556fb08a9f7e4ca02c96059f))
    - bump versions ([`ac05404`](https://github.com/ZetaNumbers/wasm4-rs/commit/ac05404fc96f0089d40dd55f238da870f683526f))
    - add preview link ([`68aab26`](https://github.com/ZetaNumbers/wasm4-rs/commit/68aab26c7facf37155dae3244aafb1740c2dd2a2))
    - add `include_sprites!` macro ([`06dc6af`](https://github.com/ZetaNumbers/wasm4-rs/commit/06dc6afedf6ea051c5927fd06f0b7fd84a6bb55b))
    - add `bitflags` crate ([`bc8c6cc`](https://github.com/ZetaNumbers/wasm4-rs/commit/bc8c6cc9fccfabda778d080255d698eaeb97809b))
    - naming ([`d2fcdeb`](https://github.com/ZetaNumbers/wasm4-rs/commit/d2fcdeb73c64d1877017d438d4d1aeaebea41c9f))
    - `SpriteView` lifetime ([`af5de1c`](https://github.com/ZetaNumbers/wasm4-rs/commit/af5de1cb924462858946620cc41902e289a96e68))
    - lifetime elision ([`caf2814`](https://github.com/ZetaNumbers/wasm4-rs/commit/caf28141b67e048b28d7506391c1b419896eee7d))
    - add default type parameter for `Sprite` ([`f365438`](https://github.com/ZetaNumbers/wasm4-rs/commit/f3654385eaf055c7c1e70660a24f24758d80b407))
    - remove prelude ([`4340269`](https://github.com/ZetaNumbers/wasm4-rs/commit/4340269a340c2da865b0f7c143fbf0306d0ab8ae))
    - fix markdown headers ([`e2a6345`](https://github.com/ZetaNumbers/wasm4-rs/commit/e2a63457da10b8083652272a60e4bb7d59250efe))
    - add `blit` support ([`883fd55`](https://github.com/ZetaNumbers/wasm4-rs/commit/883fd55ae101d7da61c8a0dd163eae3494bc8463))
    - update prelude ([`c993b2b`](https://github.com/ZetaNumbers/wasm4-rs/commit/c993b2bad68b8a9b5c68d45c9ca000504dc04014))
    - privatize utils module ([`f8c3bd8`](https://github.com/ZetaNumbers/wasm4-rs/commit/f8c3bd886712deb15f77733bbb55b92ed14d09d6))
    - add `Framebuffer::as_cells` ([`b27db46`](https://github.com/ZetaNumbers/wasm4-rs/commit/b27db4694664ff448172d40ebc4cdd5683017d29))
    - `Framebuffer`'s methods ([`fe8a08a`](https://github.com/ZetaNumbers/wasm4-rs/commit/fe8a08a0a307e194a7bbfdd2ba94a6855bf13032))
    - framebuffer's byte length ([`7aeec95`](https://github.com/ZetaNumbers/wasm4-rs/commit/7aeec9545d97bddf7a56d7ed8896c28ac64c8a42))
    - Add basic framebuffer support ([`03dd8b1`](https://github.com/ZetaNumbers/wasm4-rs/commit/03dd8b1d3bd064dae96b11fe9541ade2127c27eb))
</details>

## 0.0.3 (2022-01-02)

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

### Bug Fixes

 - <csr-id-74390f243edfeab213bc40e2ed7b12f008f1efec/> copy example's source into wasm4 docs

### Documentation

 - <csr-id-7b490feace43670f3f2100595ab6f0a1ee988d62/> add changelog for `wasm4`

### New Features

 - <csr-id-7a1d0114338f2f9c33580731ada2d348b9a5abbc/> add runtime creation functionality

### refactor (BREAKING)

 - <csr-id-233c4f95d0e7af696a6f19313257d579d4ce45f3/> change to the new experimental api

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 15 commits contributed to the release over the course of 18 calendar days.
 - 6 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release wasm4 v0.0.3 ([`6ac01fc`](https://github.com/ZetaNumbers/wasm4-rs/commit/6ac01fc6038cfb7eed61e1d9d36126cfdbf1c38a))
    - regenerate and adjust docs ([`fc71325`](https://github.com/ZetaNumbers/wasm4-rs/commit/fc71325e4a4b43f342828370c960357ac1d0f583))
    - copy example's source into wasm4 docs ([`74390f2`](https://github.com/ZetaNumbers/wasm4-rs/commit/74390f243edfeab213bc40e2ed7b12f008f1efec))
    - Release wasm4-sys v0.1.0, wasm4 v0.0.3 ([`9a8c498`](https://github.com/ZetaNumbers/wasm4-rs/commit/9a8c498c6ebff2e2a2520a74defaf407fc39f36f))
    - regenerate and adjust changelog ([`ceaee04`](https://github.com/ZetaNumbers/wasm4-rs/commit/ceaee049373326d74f9fffe14f9b7d13e87cc69e))
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

