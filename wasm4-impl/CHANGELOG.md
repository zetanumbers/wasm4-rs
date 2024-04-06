# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Chore

 - <csr-id-6a833a7d3ecd8ee045441892e1c504993c0e5c65/> update `wasm-impl` local dependencies

### Documentation

 - <csr-id-60b3c5307b97a414e492d1e429f3d1de068411f3/> document unsafe functions and fix clippy warnings
   In particular added documentation to unsafe functions with safety sections.
 - <csr-id-9deecf48578e05831dafe18131b3c0ebff7377e2/> disable documentation for `wasm-impl`

### Refactor

 - <csr-id-1546d4800d74a57a63ea273aa03473fce1ebca2b/> fix `cargo check` warnings
 - <csr-id-233cb7870c80b14a4a8f5394eab6b0c8e0586b28/> move include_sprites! implementation into its module

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 768 calendar days.
 - 810 days passed between releases.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Update `wasm-impl` local dependencies ([`6a833a7`](https://github.com/zetanumbers/wasm4-rs/commit/6a833a7d3ecd8ee045441892e1c504993c0e5c65))
    - Document unsafe functions and fix clippy warnings ([`60b3c53`](https://github.com/zetanumbers/wasm4-rs/commit/60b3c5307b97a414e492d1e429f3d1de068411f3))
    - Fix `cargo check` warnings ([`1546d48`](https://github.com/zetanumbers/wasm4-rs/commit/1546d4800d74a57a63ea273aa03473fce1ebca2b))
    - `include_sprites!` is now gated by `include-sprites` feature ([`9deecf4`](https://github.com/zetanumbers/wasm4-rs/commit/9deecf48578e05831dafe18131b3c0ebff7377e2))
    - Move include_sprites! implementation into its module ([`233cb78`](https://github.com/zetanumbers/wasm4-rs/commit/233cb7870c80b14a4a8f5394eab6b0c8e0586b28))
</details>

## 0.1.2 (2022-01-17)

### Chore

 - <csr-id-3ffbbb4e106ce545fcf0b8a1fa36e40e3384afdc/> bump versions

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release wasm4-impl v0.1.2, wasm4 v0.1.2 ([`d833a30`](https://github.com/zetanumbers/wasm4-rs/commit/d833a3084f6eb490fc9cff7f10e0e37696cdf0e2))
    - Bump versions ([`3ffbbb4`](https://github.com/zetanumbers/wasm4-rs/commit/3ffbbb4e106ce545fcf0b8a1fa36e40e3384afdc))
</details>

## 0.1.1 (2022-01-17)

### Chore

 - <csr-id-2bed0608fe6d1f95412b53e4b2a100c6d9413898/> fix crates' descriptions
 - <csr-id-7e163bdedd898202b4009fd19a79e286592174a0/> fix license

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 3 commits contributed to the release.
 - 3 days passed between releases.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release wasm4-sys v0.1.1, wasm4-impl v0.1.1, wasm4 v0.1.1 ([`9587707`](https://github.com/zetanumbers/wasm4-rs/commit/958770778205fcf22318ffb4a25dc359baa0513a))
    - Fix crates' descriptions ([`2bed060`](https://github.com/zetanumbers/wasm4-rs/commit/2bed0608fe6d1f95412b53e4b2a100c6d9413898))
    - Fix license ([`7e163bd`](https://github.com/zetanumbers/wasm4-rs/commit/7e163bdedd898202b4009fd19a79e286592174a0))
</details>

## 0.1.0 (2022-01-14)

### Chore

 - <csr-id-a0f0eb4c388e0b91a9edda291aa61f10e3388229/> update changelogs
 - <csr-id-ac05404fc96f0089d40dd55f238da870f683526f/> bump versions

### New Features

 - <csr-id-06dc6afedf6ea051c5927fd06f0b7fd84a6bb55b/> add `include_sprites!` macro

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release wasm4-impl v0.1.0 ([`2031d7b`](https://github.com/zetanumbers/wasm4-rs/commit/2031d7b5dfe38b8a5394942e1777811e5c70812b))
    - Update changelogs ([`a0f0eb4`](https://github.com/zetanumbers/wasm4-rs/commit/a0f0eb4c388e0b91a9edda291aa61f10e3388229))
    - Bump versions ([`ac05404`](https://github.com/zetanumbers/wasm4-rs/commit/ac05404fc96f0089d40dd55f238da870f683526f))
    - Add `include_sprites!` macro ([`06dc6af`](https://github.com/zetanumbers/wasm4-rs/commit/06dc6afedf6ea051c5927fd06f0b7fd84a6bb55b))
</details>

