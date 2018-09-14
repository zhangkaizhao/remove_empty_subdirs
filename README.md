# remove\_empty\_subdirs

Recursively remove empty sub-directories.

Note:

* Hidden directories which start with ".", e.g. ".git" are ignored.
* Permission denied directories and their sub-directories are ignored.

## Usage

### Library

Add the library as a dependency to your project by inserting

```toml
remove_empty_subdirs = "0.1.1"
```

into the `[dependencies]` section of your `Cargo.toml` file.

Then use it in the source code of your project. For example:

```rust
extern crate remove_empty_subdirs;

use std::path::Path;

use remove_empty_subdirs::remove_empty_subdirs;

fn main() {
    let path = Path::new("test_dir");
    remove_empty_subdirs(path).unwrap();
}
```

### Executable

First, build the executable in `examples/` directory:

```sh
cargo build --release --examples
```

Then copy the built executable `target/release/examples/remove_empty_subdirs` to anywhere you like.

Run the executable:

```sh
/path/to/remove_empty_subdirs --help
```

to get the help for usage.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
