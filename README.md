[![License](https://img.shields.io/crates/l/embed-file.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/embed-file.svg)](https://crates.io/crates/embed-file)
[![Docs.rs](https://docs.rs/embed-file/badge.svg)](https://docs.rs/embed-file)

# embed-file

A simplified version of [rust-embed][] for single files instead of folders.
Embeds files content into the binary in release mode, but loads it from the fs in debug.

The goal of is to avoid unnecessary recompilations when an included by e.g. [`include_str`]
file change.

## Usage

```rust,no_run
let my_string = embed_file::embed_string!("path/to/my-text.txt");
let my_image = embed_file::embed_bytes!("path/to/my-image.jpg");
```

## Contributing

- please run [.pre-commit.sh] before sending a PR, it will check everything

## License

This project is licensed under the [MIT license][license].

[.pre-commit.sh]: https://github.com/{{github-user}}/{{project-name}}/blob/main/pre-commit.sh
[license]: https://github.com/{{github-user}}/{{project-name}}/blob/main/LICENSE
[rust-embed]: https://github.com/pyrossh/rust-embed/
