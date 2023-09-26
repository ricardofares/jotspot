mod annotation;
mod metadata;
mod ui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        match metadata::read_annotations() {
            Ok(annotations) => ui::run_annotate_tui(annotations),
            Err(e) => eprintln!("Couldn't read annotations: {}", e),
        }
    } else {
        metadata::annotate(&args.join(" "));
    }
}
