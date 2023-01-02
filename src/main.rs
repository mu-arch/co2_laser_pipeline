use std::path::{Path, PathBuf};
use std::{fs, thread};
use std::error::Error;
use std::fs::DirEntry;
use notify::{Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use std::time::Duration;
use vtracer::{ColorMode, Config, Hierarchical};
use notify_debouncer_mini::new_debouncer;
use crate::errors::AppErrorInternal;


#[path = "lib/errors.rs"]
mod errors;

fn main() {

    let handler = thread::spawn(|| {
        watch_directory_loop()
    });

    loop {
        thread::sleep(Duration::from_secs(999))
    }

}

fn watch_directory_loop() {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(250), None, tx).unwrap();

    let path = Path::new("/home/vuen/Desktop/kentlasers_system/in");

    debouncer
        .watcher()
        .watch(path, RecursiveMode::Recursive)
        .unwrap();

    for events in rx {
        let _ = inspect_filelist(path);
    }
}

fn inspect_filelist(path: &Path) -> Result<(), Box<dyn Error>> {
    let mut entries = fs::read_dir(path)?;

    for file in entries {
        let file = match file {
            Ok(v) => v,
            Err(_) => {
                continue
            }
        };

        println!("- Started SVG processing job for: {:?}", &file.path() );

        let e = process_with_vtracer(&file);

        let response = match e {
            Ok(_) => "Job succeeded".to_owned(),
            Err(e) => format!("ERROR: {e}")
        };

        println!("- {}", response);

        fs::remove_file(file.path())?;
    }

    Ok(())
}

fn process_with_vtracer(file: &DirEntry) -> Result<(), Box<dyn Error>>{

    let config = Config {
        input_path: file.path(),
        output_path: PathBuf::from("/home/vuen/Desktop/kentlasers_system/out/vectorized.svg"),
        color_mode: ColorMode::Color,
        hierarchical: Hierarchical::Stacked,
        filter_speckle: 4,
        color_precision: 6,
        layer_difference: 16,
        mode: Default::default(),
        corner_threshold: 60,
        length_threshold: 4.0,
        splice_threshold: 45,
        max_iterations: 10,
        path_precision: Some(8),
    };

    vtracer::convert_image_to_svg(config)?;

    Ok(())
}