# Safe Rust bindings for `libvips`

## To-do

### `vips-sys`

- [ ] Fix `warning: 'extern' block uses type 'u128', which is not FFI-safe`
- [ ] Create Allowlist for FFI bindings

### `vips-rs`

- [ ] Write a wrapper for `VipsImage` and `call`
- [ ] Create an Option type
- [ ] Adapt operator generating script for Rust
- [ ] Abstract away `VIPS_INIT` and `vips_shutdown` as much as possible
    - [x] `vips_init() -> Result<_,_>`
    - [x] `VipsHandle`
    - [x] `ensure_vips_init_or_exit()`
    - [ ] `vips_main` decorator macro (`vips_init_or_exit` before fn,
          `vips_shutdown` after)
- [ ] Merge vips_sys and vips_rs (this repo)
- [ ] Write rudimentary test suite
    - [ ] leak checks
    - [ ] basic functionality checks
- [ ] Publish crate