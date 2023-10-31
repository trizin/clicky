mod keymap;
mod sound_config;
extern crate device_query;
extern crate rodio;

use std::{io::Cursor, sync::Arc, time::Instant};

use device_query::{DeviceQuery, DeviceState};
use rand::{seq::SliceRandom, Rng};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use sound_config::cache_sounds;

static AUDIO_PATH: &str = "./output";

fn play_key_sound(raw_data: Arc<Vec<u8>>, stream_handle: &OutputStreamHandle) -> Sink {
    let start_time = Instant::now();
    let sink = Sink::try_new(stream_handle).unwrap();

    let base_volume = 1.0;
    let mut rng = rand::thread_rng();

    let random_variation = rng.gen_range(0.50..1.50);
    let speed_factor: f32 = rng.gen_range(0.8..=1.4);

    sink.set_volume(base_volume * random_variation);
    sink.set_speed(speed_factor);

    let cloned_data = raw_data.as_ref().to_vec();
    let cursor = Cursor::new(cloned_data);
    let source = Decoder::new(cursor).unwrap();

    sink.append(source);
    let duration = start_time.elapsed();
    sink
}

fn main() {
    let available_keys = sound_config::get_key_numbers_from_folder(AUDIO_PATH);
    let device_state = DeviceState::new();
    let mut last_keys = device_state.get_keys();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut active_sinks: Vec<Sink> = Vec::new();
    let mut keymap = keymap::get_key_map();
    let sound_cache = cache_sounds(AUDIO_PATH);

    println!("Start typing!");

    loop {
        let current_keys = device_state.get_keys();

        for key in &current_keys {
            if !last_keys.contains(key) {
                let key_string = format!("{}", key);
                let mut keymap_number: i32 = keymap.get(&key_string).unwrap_or(&65).clone();

                if !available_keys.contains(&keymap_number) {
                    let keys_vec: Vec<_> = available_keys.iter().collect();
                    keymap_number = keys_vec
                        .choose(&mut rand::thread_rng())
                        .cloned()
                        .unwrap()
                        .clone();

                    keymap.remove(&key_string);
                    keymap.insert(key_string.clone(), keymap_number);
                }

                let filename = format!("./output/key_sound_{}.mp3", keymap_number);
                let source = sound_cache.get(filename.as_str()).expect("Cache error");
                let sink = play_key_sound(source.clone(), &stream_handle);
                active_sinks.push(sink);
            }
        }

        last_keys = current_keys;

        active_sinks.retain(|sink| !sink.empty());

        // Sleep to prevent 100% CPU usage.
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
