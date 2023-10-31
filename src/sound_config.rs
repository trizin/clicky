use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Read;
use std::sync::Arc;

pub fn get_key_numbers_from_folder(path: &str) -> HashSet<i32> {
    let mut key_numbers = HashSet::new();

    // Iterate over entries in the directory
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name().into_string().unwrap_or_default();

                // Check if the filename matches the pattern 'key_sound_x.mp3'
                if let Some(captured) = filename.strip_prefix("key_sound_") {
                    if let Some(number) = captured.strip_suffix(".mp3") {
                        if let Ok(number) = number.parse::<i32>() {
                            key_numbers.insert(number);
                        }
                    }
                }
            }
        }
    }

    key_numbers
}

pub struct SoundCache {
    cache: HashMap<String, Arc<Vec<u8>>>,
}

impl SoundCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn load(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let arc_buffer = Arc::new(buffer);
        self.cache.insert(filename.to_string(), arc_buffer);
    }

    pub fn get(&self, filename: &str) -> Option<&Arc<Vec<u8>>> {
        self.cache.get(filename)
    }
}

pub fn cache_sounds(path: &str) -> SoundCache {
    let mut sound_cache = SoundCache::new();

    let files = get_key_numbers_from_folder(path);

    for number in files {
        let file_path = format!("{}/key_sound_{}.mp3", path, number);
        sound_cache.load(&file_path);
    }

    return sound_cache;
}
