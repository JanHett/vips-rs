# Safe Rust bindings for `libvips`

For the moment, this crate requires libvips 8.13 or higher to be installed on the system.

## To-do

### `vips-sys`

- [ ] Fix `warning: 'extern' block uses type 'u128', which is not FFI-safe`
- [ ] Create Allowlist for FFI bindings

### `vips-rs`

- [ ] Complete the wrapper for `VipsImage`
    - [x] Constructors
    - [ ] Image output
    - [ ] Image properties
    - [ ] `[]` and `()` operators as per the C++ bindings?
- [ ] Generate operators (via macro?)
- [ ] Abstract away `VIPS_INIT` and `vips_shutdown` as much as possible
    - [x] `vips_init() -> Result<_,_>`
    - [x] `VipsHandle`
    - [x] `ensure_vips_init_or_exit()`
    - [ ] `vips_main` decorator macro (`vips_init_or_exit` before fn,
          `vips_shutdown` after)
- [x] Merge vips_sys and vips_rs (this repo)
- [ ] Write rudimentary test suite
    - [ ] leak checks
    - [ ] basic functionality checks
- [ ] Ship Vips with this crate
- [ ] Publish crate