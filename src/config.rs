use std::fs;
use std::io::Read;

// Load cargo.
use serde::{Deserialize};
use serde_xml_rs::{from_str, to_string};

#[derive(Deserialize, Debug)]
struct ConfigData {
    title: String,
    description: String,
    changenote: String,
    fileid: String,
}

pub struct Config {
    data: ConfigData,
}

impl Config {
    /**
     * Instanciate new Config instance.
     */
    pub fn new(path: &str) -> Config {
        let data: ConfigData = Self::load_file(path);

        Config {
            data: ConfigData {
                title: data.title,
                description: data.description,
                changenote: data.changenote,
                fileid: data.fileid,
            },
        }
    }

    /**
     * Return value for a given element name.
     */
    pub fn get_value(&self, name: &str) -> String {
        match name {
            "title" => return self.data.title.clone(),
            "description" => return self.data.description.clone(),
            "changenote" => return self.data.changenote.clone(),
            "fileid" => return self.data.fileid.clone(),
            _ => panic!("Config paramater {} not found", name),
        };
    }

    /**
     * Load XML file into ConfigData struct.
     */
    fn load_file(path: &str) -> ConfigData {
        let mut file = fs::File::open(path).unwrap();
        let mut text = String::new();

        file.read_to_string(&mut text).unwrap();

        let data: ConfigData = from_str(&text).unwrap();
        data
    }
}
