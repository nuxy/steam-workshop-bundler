#![allow(unused_variables)]

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

// Load cargo.
use image::RgbImage;
use uuid::Uuid;
use which::which;

// Config mod.
mod config;
use config::Config;

// Configurable.
const PREVIEW_IMG_HEIGHT: u32 = 356;
const PREVIEW_IMG_WIDTH:  u32 = 635;

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
 * Create archive of project sources.
 */
pub fn create_bundle(workshop: &str, public: &bool) -> String {
    let proj_path = format!("{}/Workshop/{}", get_cwd_path(), workshop);

    if Path::new(&proj_path).is_dir() {
        let cmd_bin = get_bin_path("7za") + get_bin_ext();

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
                &(format!("{}/{}.pak", build_path, workshop)),
                &(format!("@\"{}/MANIFEST\"", proj_path)),
                &(format!("{}/LICENSE", proj_path)),
                &(format!("{}/VERSION", proj_path)),
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
 * Create workshop project source.
 */
pub fn create_workshop(name: &str) {
    let proj_path = format!("{}/Workshop/{}", get_cwd_path(), name);

    fs::create_dir_all(&proj_path).expect("Failed to create directory");

    let xml_file = format!("{}/config.xml", &proj_path);
    let img_file = format!("{}/preview.png", &proj_path);
    let man_file = format!("{}/MANIFEST", &proj_path);

    // Write config values to XML.
    let config = Config::new(&xml_file);
    config.write_file();

    // Generate preview image.
    let image = RgbImage::new(PREVIEW_IMG_WIDTH, PREVIEW_IMG_HEIGHT);
    image.save(&img_file).unwrap();

    // Create empty manifest.
    fs::write(man_file, "").expect("Failed to create file");
}

/**
 * Upload project sources using SteamCMD.
 */
pub fn publish(build_path: &str, username: &str, password: &str) -> bool {
    if Path::new(&build_path).is_dir() {
        let cmd_bin = get_bin_path("steamcmd") + get_bin_ext();

        Command::new(cmd_bin)
            .args(&[
                "+login",
                &(format!("'{}'", username)),
                &(format!("'{}'", password)),
                "+workshop_build_item",
                &(format!("{}/mod.vcf", build_path)),
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
    let title = config.get_value("title");
    let description = config.get_value("description");
    let changenote = config.get_value("changenote");
    let tags = config.get_value("tags");
    let fileid = config.get_value("fileid");

    let visible = if *public { "0" } else { "3" }; // 0: private, 3: public

    // Output VDF format.
    let content = format! {"
\"workshopitem\"
{{
    \"appid\"           \"{appid}\"
    \"contentfolder\"   \"{build_path}\"
    \"previewfile\"     \"{proj_path}/preview.png\"
    \"visibility\"      \"{visible}\"
    \"title\"           \"{title}\"
    \"description\"     \"{description}\"
    \"changenote\"      \"{changenote}\"
    \"tags\"            \"{tags}\"
    \"publishedfileid\" \"{fileid}\"
}}
"};

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
    let file = file_name.to_string() + get_bin_ext();

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
