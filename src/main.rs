mod annotation;
mod ui;

use std::convert::From;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const ANNOTATIONS_FILENAME: &str = ".annotations";

fn read_annotations_file() -> Result<Vec<String>, io::Error> {
    match File::open(ANNOTATIONS_FILENAME) {
        Ok(file) => {
            let lines: Result<Vec<String>, io::Error> = BufReader::new(file).lines().collect();
            Ok(lines?)
        }
        _ => {
            File::create(ANNOTATIONS_FILENAME)?;
            Ok(Vec::<String>::new())
        }
    }
}

fn read_annotations() -> Vec<annotation::Annotation> {
    let annotations_result = read_annotations_file();

    match annotations_result {
        Ok(lines) => lines
            .iter()
            .map(|annotation_str| annotation::Annotation::from(annotation_str))
            .collect(),
        Err(err) => {
            panic!(
                "Annotations file {} could not be read: {}",
                ANNOTATIONS_FILENAME, err
            );
        }
    }
}
fn annotate(content: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(ANNOTATIONS_FILENAME)
        .unwrap();

    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Invalid system time!")
        .as_millis();

    if let Err(e) = writeln!(file, "{} {}", created_at, content) {
        eprintln!("Annotation failed: {}", e);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        ui::run_annotate_tui(read_annotations());
    } else {
        annotate(args.join(" "));
    }
}
