//! Soundcloud downloader made using Rust
//!
//! ## Install
//! If you have Rust: `cargo install sndcld`
//!
//! ## Usage
//!
//! ```bash
//! sndcld 0.2.1
//! DEADBLACKCLOVER <deadblackclover@protonmail.com>
//! Soundcloud downloader made using Rust
//!
//! USAGE:
//!     sndcld [SUBCOMMAND]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! SUBCOMMANDS:
//!     help        Prints this message or the help of the given subcommand(s)
//!     playlist    Download playlist
//!     song        Download a song
//!     token       Adding a token
//! ```
use clap::{App, Arg, SubCommand};

mod config;
mod downloader;

use config::Config;

fn main() {
    let matches = App::new("sndcld")
        .version("0.2.1")
        .author("DEADBLACKCLOVER <deadblackclover@protonmail.com>")
        .about("Soundcloud downloader made using Rust")
        .subcommand(
            SubCommand::with_name("token").about("Adding a token").arg(
                Arg::with_name("TOKEN")
                    .help("Sets the token to use")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("song").about("Download a song").arg(
                Arg::with_name("URL")
                    .help("Soundcloud song URL")
                    .required(true)
                    .index(1),
            ),
        )
        .subcommand(
            SubCommand::with_name("playlist")
                .about("Download playlist")
                .arg(
                    Arg::with_name("URL")
                        .help("Soundcloud playlist URL")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let config = Config::init();

    if let Some(matches) = matches.subcommand_matches("token") {
        let token = matches.value_of("TOKEN").expect("Token not found");
        config
            .save_token(String::from(token))
            .expect("Error saving file");
    }

    if let Some(matches) = matches.subcommand_matches("song") {
        let url = matches.value_of("URL").unwrap();
        downloader::song(&config, String::from(url)).expect("Failed to get song data");
    }

    if let Some(matches) = matches.subcommand_matches("playlist") {
        let url = matches.value_of("URL").unwrap();
        downloader::playlist(&config, String::from(url)).expect("Failed to get playlist data");
    }
}
