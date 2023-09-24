mod annotation;

use annotation::Annotation;

use cursive::view::Scrollable;
use cursive::views::{Dialog, LinearLayout, ScrollView, TextView};

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

fn read_annotations() -> Vec<Annotation> {
    let annotations_result = read_annotations_file();

    match annotations_result {
        Ok(lines) => lines
            .iter()
            .map(|annotation_str| Annotation::from(annotation_str))
            .collect(),
        Err(err) => {
            panic!(
                "Annotations file {} could not be read: {}",
                ANNOTATIONS_FILENAME, err
            );
        }
    }
}

fn build_annotation_text(annotation: &Annotation) -> LinearLayout {
    LinearLayout::horizontal()
        .child(TextView::new(annotation.format_created_at() + " | "))
        .child(TextView::new(annotation.content.clone()))
}

fn build_annotations_layout(annotations: &[Annotation]) -> ScrollView<LinearLayout> {
    if annotations.len() == 0 {
        LinearLayout::vertical()
            .child(TextView::new("You have not registered any annotation!"))
            .child(TextView::new("Try: annotate [text]").center())
            .scrollable()
    } else {
        annotations
            .iter()
            .fold(LinearLayout::vertical(), |layout, annotation| {
                layout.child(build_annotation_text(annotation))
            })
            .scrollable()
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

fn run_annotate_tui() {
    let annotations = read_annotations();
    let mut siv = cursive::default();

    siv.add_layer(Dialog::around(build_annotations_layout(&annotations)).title("Annotations"));
    siv.run();
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        run_annotate_tui();
    } else {
        annotate(args.join(" "));
    }
}
