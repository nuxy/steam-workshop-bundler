#![allow(unused_variables)]

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Load cargo.
use uuid::Uuid;
use which::which;

// Config mod.
mod config;
use config::Config;

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
pub fn create_bundle(workshop: &str, public: &bool) -> String {
    let proj_path = format!("{}/Workshop/{}", get_cwd_path(), workshop);

    if Path::new(&proj_path).is_dir() {
        let cmd_bin = format!("{}{}", get_bin_path("7za"), get_bin_ext());

        // Create build directory.
        let build_uuid = Uuid::new_v4().to_string();
        let build_path = format!("{}/{}", get_tmp_path(), build_uuid);

        fs::create_dir(&build_path).expect("Failed to create directory");

        // .. 7za archive.
        Command::new(cmd_bin)
            .args(&[
                "a",
                "-tzip",
                "-mx0",
                "{build_path}/{workshop}.pak",
                "@\"{proj_path}/MANIFEST\"",
                "{proj_path}/LICENSE",
                "{proj_path}/VERSION",
            ])
            .output()
            .expect("Failed to execute process");

        // .. dependencies.
        create_vdf(&build_path, &proj_path, &public);

        return build_path;
    }

    panic!("Workshop \"{}\" not found. Exiting.", workshop);
}

/**
 * Upload package sources using SteamCMD.
 */
pub fn publish(build_path: &str, username: &str, password: &str) -> bool {
    if Path::new(&build_path).is_dir() {
        let cmd_bin = format!("{}{}", get_bin_path("steamcmd"), get_bin_ext());

        Command::new(cmd_bin)
            .args(&[
                "+login",
                "{username}",
                "{password}",
                "+workshop_build_item",
                "{build_path}/mod.vcf",
                "+quit",
            ])
            .output()
            .expect("Failed to execute process");

        // Cleanup build sources.
        fs::remove_dir_all(build_path).expect("Failed to remove directory");

        return true;
    }

    panic!("Build directory \"{}\" not found. Exiting.", build_path);
}

/**
 * Create Steam workshop VDF reference.
 */
fn create_vdf(build_path: &str, proj_path: &str, public: &bool) {
    let xml_file = format!("{}/config.xml", get_cwd_path());
    let vdf_path = format!("{}/mod.vdf", get_tmp_path());

    // Load config values from XML.
    let config = Config::new(&xml_file);

    let appid = config.get_value("appid");
    let name = config.get_value("name");
    let description = config.get_value("description");
    let changenote = config.get_value("changenote");
    let tags = config.get_value("tags");
    let fileid = config.get_value("fileid");

    let visible = if *public { "0" } else { "3" };

    // Output VDF format.
    let content = r#"
"workshopitem"
{
    "appid"           "{appid}"
    "contentfolder"   "{build_path}"
    "previewfile"     "{proj_path}/preview.png"
    "visibility"      "{visible}"
    "title"           "{title}"
    "description"     "{description}"
    "changenote"      "{changenote}"
    "tags"            "{tags}"
    "publishedfileid" "{fileid}"
}
"#;

    fs::write(vdf_path, content).expect("Failed to write VDF data");
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
    let file = format!("{}{}", file_name, get_bin_ext());

    which(file).unwrap().display().to_string()
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
