use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Load cargo.
use uuid::Uuid;
use which::which;

/**
 * Check OS-specific dependencies.
 */
pub fn check_deps(file_names: &[&str]) {
    for file_name in file_names {
        let result = get_bin_path(file_name);

        if result == "" {
            panic!("{}{} is not installed. Exiting.", file_name, get_bin_ext());
        }
    }
}

/**
 * Create archive, project sources.
 */
pub fn create_bundle(workshop: String, public: bool) {
    let proj_path = format!("{}/Workshop/{}", get_cwd_path(), workshop);

    if Path::new(&proj_path).is_dir() {
        let cmd = format!("{}{}", get_bin_path("7za"), get_bin_ext());

        // Create build directory..
        let build_uuid = Uuid::new_v4().to_string();
        let build_path = format!("{}/{}", get_tmp_path(), build_uuid);

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
    if env::consts::OS == "windows" {
        ".exe"
    } else {
        ""
    }
}

/**
 * Return OS-supported binary path.
 */
fn get_bin_path(file_name: &str) -> String {
    which(file_name).unwrap().display().to_string()
}

/**
 * Return path to current directory.
 */
fn get_cwd_path() -> String {
    env::current_dir().unwrap().display().to_string()
}

/**
 * Return path to OS temp directory.
 */
fn get_tmp_path() -> String {
    env::temp_dir().display().to_string()
}
