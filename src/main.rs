// Port of https://github.com/nuxy/MiscreatedMods-DOA/blob/master/publish.sh

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Load cargo.
use clap::{AppSettings, Parser};
use uuid::Uuid;
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
    check_deps(&["steamcmd", "git", "7za"]);

    let args = Cli::parse();

    pack_files(args.workshop);
}

/**
 * Check OS-specific dependencies.
 */
fn check_deps(file_names: &[&str]) {
    for file_name in file_names {
        let result = get_bin_path(file_name);

        if result == "" {
            panic!("{}{} is not installed. Exiting.", file_name, get_bin_ext());
        }
    }
}

/**
 * Create archive of project files.
 */
fn pack_files(workshop: String) {
    let cwd = env::current_dir().unwrap().display().to_string();
    let tmp = env::temp_dir().display().to_string();

    let proj_path = format!("{}/Workshop/{}", cwd, workshop);

    if Path::new(&proj_path).is_dir() {
        let cmd = format!("{}{}", get_bin_path("7za"), get_bin_ext());

        // Create build directory..
        let build_uuid = Uuid::new_v4().to_string();
        let build_path = format!("{}/{}", tmp, build_uuid);

        fs::create_dir(build_path).unwrap_or_else(|error| {
            panic!("Failed to create directory: {:?}", error);
        });

        // .. then 7za archive.
        Command::new(cmd)
            .args(&[
                "a",
                "-tzip",
                "-mx0",
                "{build_path}/{workshop}.pak",
                "@\"{proj_path}/MANIFEST\"",
                "LICENSE",
                "VERSION",
            ])
            .output()
            .expect("failed to execute process");
    } else {
        panic!("Workshop \"{}\" not found. Exiting.", workshop);
    }
}

/**
 * Return OS-supported binary entension.
 */
fn get_bin_ext() -> &'static str {
    return if env::consts::OS == "windows" {
        ".exe"
    } else {
        ""
    };
}

/**
 * Return OS-supported binary path.
 */
fn get_bin_path(file_name: &str) -> String {
    return which(file_name).unwrap().display().to_string();
}
