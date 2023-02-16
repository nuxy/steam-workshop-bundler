use std::fs;
use std::fs::File;
use std::path::Path;

// Load cargo.
use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Deserialize, Serialize, Debug, Default)]
struct ConfigData {
    appid: String,
    title: String,
    description: String,
    changenote: String,
    tags: String,
    fileid: String,
}

pub struct Config {
    data: ConfigData,
    file: String,
}

impl Config {
    /**
     * Instanciate new Config instance.
     */
    pub fn new(file: &str) -> Self {
        if Path::new(&file).exists() {
            let data: ConfigData = Self::load_data(&file);

            Self {
                data: ConfigData {
                    appid: data.appid,
                    title: data.title,
                    description: data.description,
                    changenote: data.changenote,
                    tags: data.tags,
                    fileid: data.fileid,
                },
                file: file.to_string(),
            }
        } else {
            Self {
                data: ConfigData::default(),
                file: file.to_string(),
            }
        }
    }

    /**
     * Return value for a given element name.
     */
    pub fn get_value(&self, name: &str) -> String {
        match name {
            "appid" => return self.data.appid.clone(),
            "title" => return self.data.title.clone(),
            "description" => return self.data.description.clone(),
            "changenote" => return self.data.changenote.clone(),
            "tags" => return self.data.tags.clone(),
            "fileid" => return self.data.fileid.clone(),
            _ => panic!("Config parameter {} not found", name),
        };
    }

    /**
     * Output XML config, empty values if new.
     */
    pub fn output_file(&self, path: &str) {
        let values = to_string(&self.data).unwrap();

        // Output XML format.
        let content = r#"<config>{values}</config>"#;

        let file = File::create(&path).expect("Failed to create XML");

        fs::write(path, content).expect("Failed to write XML data");
    }

    /**
     * Load XML file into ConfigData struct.
     *
     * Supported format:
     *   <config>
     *       <appid />
     *       <title />
     *       <description />
     *       <changenote />
     *       <tags />
     *       <fileid />
     *   </config>
     */
    fn load_file(path: &str) -> ConfigData {
        let text = fs::read_to_string(path).expect("Failed to read file");

        let data: ConfigData = from_str(&text).expect("Invalid XML format");
        data
    }
}
