[![License](https://img.shields.io/crates/l/embed-file.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/embed-file.svg)](https://crates.io/crates/embed-file)
[![Docs.rs](https://docs.rs/embed-file/badge.svg)](https://docs.rs/embed-file)

<!-- cargo-sync-readme start -->

# embed-file

A simplified version of [rust-embed][] for single files instead of folders.
Embeds files content the binary in release mode, but loads it from the fs in debug.

The goal of is to avoid unnecessary recompilations when an included by e.g. [`include_str`]
file change.

## Usage

```rust
let my_file_contents = embed_file::embed_string!("path/to/my_file.txt");
println!("{}", my_file_contents);
```
[rust-embed]: https://github.com/pyrossh/rust-embed/

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
