# sndcld

![crates.io](https://img.shields.io/crates/v/sndcld.svg)
![docs.rs](https://docs.rs/sndcld/badge.svg)

Soundcloud downloader made using Rust

## Install

If you have Rust: `cargo install sndcld`


## Usage

```bash
sndcld 0.2.1
DEADBLACKCLOVER <deadblackclover@protonmail.com>
Soundcloud downloader made using Rust

USAGE:
    sndcld [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    playlist    Download playlist
    song        Download a song
    token       Adding a token
```

## Build deb

```sh
cargo install cargo-deb
cargo deb
```
