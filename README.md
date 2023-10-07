# Examples

[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/vita-rust/examples)
[![GitHub Actions Build Status](https://img.shields.io/github/actions/workflow/status/vita-rust/examples/release.yml)](https://github.com/vita-rust/examples/actions/workflows/release.yml)
[![Current Release](https://img.shields.io/github/release/vita-rust/examples.svg)](https://github.com/vita-rust/examples/releases)
[![Main Commits RSS Feed](https://img.shields.io/badge/rss-commits-ffa500?logo=rss)](https://github.com/vita-rust/examples/commits/main.atom)

This repository holds examples in Rust that can be compiled as `vpk` for Sony Playstation Vita

## Prerequisites

- [Vita SDK](https://vitasdk.org/) must be installed, and `VITASDK` environment variable must be set to its location. You can add the environment variable to your `.bashrc` (or another configuration file if you are using a different shell), or you can use a tool like [direnv](https://direnv.net/), and put this in a `.envrc`.
- [cargo-vita](https://github.com/vita-rust/cargo-vita) tool is required for building `vpk`` files. Run this command to install it:
  ```sh
  cargo install cargo-vita
  ```
- [PrincessLog](https://github.com/CelesteBlue-dev/PSVita-RE-tools/tree/master/PrincessLog/build) is required for monitoring stdout/stderr from your Vita.
- [vitacompanion](https://github.com/devnoname120/vitacompanion) is required for uploading `eboot.bin`/`vpk` files to Vita, and running commands.


## Building

To build the `vpk` for every example run:

```sh
cargo vita build vpk --release
```

To build the `vpk` for `std-tests` run:

```sh
cargo vita build vpk --release --package vita-std-tests --tests
```

To build the `vpk` for any specific package:

```sh
cargo vita build vpk --release --package {PACKAGE}
```


## Running

You can automate uploading of `vpk` to `ux0:/download/`, or update `eboot.bin` for already installed `vpk`.

To upload all `vpk` artifacts, use `--upload` flag of `vpk` subcommand:

```sh
cargo vita build vpk --upload --release
```

To update a specific `eboot.bin` and run it use `--update --run` flags of `eboot` subcommand. Keep in mind that `vpk` must be installed in order for that to work:

```sh
cargo vita build eboot --update --run --release --package {PACKAGE}
```

## License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed at your option under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
