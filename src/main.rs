// Port of https://github.com/nuxy/MiscreatedMods-DOA/blob/master/publish.sh

use std::env;

// Load cargo.
use clap::{AppSettings, Parser};

// Common lib.
use steam_workshop_bundler::{check_deps, create_bundle, create_workshop, publish};

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser, Debug)]
#[clap(author, version, about = DESCRIPTION, long_about = None)]
#[clap(global_setting(AppSettings::ArgRequiredElseHelp))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]

struct Cli {
    #[clap(long, value_parser, help = "Steam account Username.")]
    username: String,

    #[clap(long, value_parser, help = "Steam account password.")]
    password: String,

    #[clap(long, value_parser, help = "Steam Guard code.")]
    guard_code: String,

    #[clap(long, value_parser, help = "Workshop name to publish.")]
    workshop: String,

    #[clap(long, action, help = "Generate workshop sources (optional).")]
    generate: bool,

    #[clap(long, action, help = "Adds workshop to Steam results (optional).")]
    public: bool,
}

/**
 * Let's get this party started.
 */
fn main() {
    check_deps(&["steamcmd", "7za"]);

    let args = Cli::parse();

    if args.generate {
        let path = create_workshop(&args.workshop);

        println!("Created workshop {} in: {}", &args.workshop, path);
    } else {
        let path = create_bundle(&args.workshop, &args.public);

        if !publish(&path, &args.username, &args.password, &args.guard_code) {
            panic!("Failed to publish Steam workshop");
        }

        println!("Success");
    }
}
