// Port of https://github.com/nuxy/MiscreatedMods-DOA/blob/master/publish.sh

use clap::{AppSettings, Parser};

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
    let args = Cli::parse();

    println!("Username: {:?}", args.username);
    println!("Password: {:?}", args.password);
    println!("Workshop: {:?}", args.workshop);
    println!("Public:   {:?}", args.public);
}
