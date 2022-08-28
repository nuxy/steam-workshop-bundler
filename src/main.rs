// Port of https://github.com/nuxy/MiscreatedMods-DOA/blob/master/publish.sh

use std::env;

// Load cargo.
use clap::{AppSettings, Parser};

// Common lib.
use steam_workshop_bundler::{check_deps, create_bundle};

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

/**
 * Let's get this party started.
 */
fn main() {
    check_deps(&["steamcmd", "git", "7za"]);

    let args = Cli::parse();

    create_bundle(args.workshop, args.public);
}
