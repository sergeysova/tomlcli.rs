# tomlcli

[![Crates.io](https://img.shields.io/crates/v/tomlcli.svg?maxAge=2592000)](https://crates.io/crates/tomlcli)
[![Crates.io](https://img.shields.io/crates/l/tomlcli.svg?maxAge=2592000)](https://github.com/sergeysova/tomlcli/blob/master/LICENSE)

Pretty print and query TOML files

## How to use

```bash
cargo install tomlcli
```

Next use `toml` or `tomlcli` binary.

```bash
# Pretty print contents of file
toml Cargo.toml

# Pretty print table
toml Cargo.toml package

# Print value in table
toml Cargo.toml package.name

# Print value in array
toml Cargo.toml package.keywords.0

# Print table in array
toml Cargo.lock package.0

# Print value in table in array
toml Cargo.lock package.0.name
```

## Todo

1. Pretty print table in field:

```toml
[[bin]]
name = "foo"
path = "./bar.rs"
```

2. Query fields with dot:

```toml
[example]
"foo.bar" = "example"
```

## License

Copyright (c) Sergey Sova. All rights reserved.
Licensed under the MIT license. See [LICENSE](https://github.com/sergeysova/tomlcli.rs/blob/master/LICENSE) file in the project root for details.
