use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn get_key_map() -> HashMap<String, i32> {
    // Load and parse the config file
    let mut config_str = String::new();
    File::open("keymap.toml")
        .unwrap()
        .read_to_string(&mut config_str)
        .unwrap();
    let config: HashMap<String, i32> = toml::from_str(&config_str).unwrap();
    return config;
}
