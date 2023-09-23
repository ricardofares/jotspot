use cursive::view::Scrollable;
use cursive::views::{Dialog, LinearLayout, ScrollView, TextView};

use std::convert::From;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const ANNOTATIONS_FILENAME: &str = ".annotations";

struct Annotation {
    /// Annotation content.
    pub content: String,

    /// Created at timestamp in milliseconds.
    pub created_at: u64,
}

impl Annotation {
    fn format_created_at(&self) -> String {
        let delta = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Invalid system delta time")
            .as_millis()
            - self.created_at as u128;

        let years = delta / (1000 * 60 * 60 * 24 * 365);
        let days = delta / (1000 * 60 * 60 * 24) % 365;
        let hours = delta / (1000 * 60 * 60) % 24;
        let minutes = delta / (1000 * 60) % 60;
        let seconds = delta / 1000 % 60;

        if years > 0 {
            format!("{:02} years ago", years)
        } else if days > 0 {
            format!("{:02} days ago", days)
        } else if hours > 0 {
            format!("{:02} hours ago", hours)
        } else if minutes > 0 {
            format!("{:02} minutes ago", minutes)
        } else if seconds > 0 {
            format!("{:02} seconds ago", seconds)
        } else {
            "Just now".to_string()
        }
    }
}

impl From<&String> for Annotation {
    fn from(string: &String) -> Annotation {
        let created_at_delim_pos = string
            .chars()
            .position(|c| c == ' ')
            .expect("No created_at delimiter position has been identified.");

        let (created_at_str, content_str) = string.split_at(created_at_delim_pos);
        let content_str = &content_str[1..]; // Skip the space

        let created_at = created_at_str
            .parse()
            .expect("From<&String> for Annotation: created_at could not be parsed.");

        Annotation {
            content: content_str.to_string(),
            created_at,
        }
    }
}

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
