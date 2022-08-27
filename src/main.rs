// Port of https://github.com/nuxy/MiscreatedMods-DOA/blob/master/publish.sh

use std::env;
use std::process;

// Load cargo.
use clap::{AppSettings, Parser};
use which::which;

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser, Debug)]
#[clap(author, version, about = DESCRIPTION, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]

struct Cli {
    #[clap(long, value_parser, help = "Steam account Username.")]
    username: String,

    #[clap(long, value_parser, help = "Steam account password.")]
    password: String,

    #[clap(long, value_parser, help = "Workshop name to publish.")]
    workshop: String,

    #[clap(long, action, help = "Adds workshop to Steam results (optional).")]
    public: bool,
}

fn main() {
    check_deps();

    let args = Cli::parse();

    println!("Username: {:?}", args.username);
    println!("Password: {:?}", args.password);
    println!("Workshop: {:?}", args.workshop);
    println!("Public:   {:?}", args.public);
}

/**
 * Check OS-specific dependencies.
 */
fn check_deps() {
    let file_ext: &str = if env::consts::OS == "windows" {
        ".exe"
    } else {
        ""
    };

    let file_names = &["steamcmd", "git", "7za"];

    for file_name in file_names {
        let result: &str = which(file_name).unwrap().as_path().display().to_string();

        if result == "" {
            println!("{file_name}{file_ext} is not installed. Exiting.");
            process::exit(1);
        }
    }
}
