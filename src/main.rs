use clap::{App, Arg, SubCommand};

mod config;
mod downloader;

use config::Config;

fn main() {
    let matches = App::new("sndcld")
        .version("0.1.0")
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
        let token = matches.value_of("TOKEN").unwrap();
        config.save_token(String::from(token));
    }

    if let Some(matches) = matches.subcommand_matches("song") {
        let url = matches.value_of("URL").unwrap();
        downloader::song(&config, String::from(url));
    }

    if let Some(matches) = matches.subcommand_matches("playlist") {
        let url = matches.value_of("URL").unwrap();
        downloader::playlist(&config, String::from(url));
    }
}
