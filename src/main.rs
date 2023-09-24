mod annotation;
mod metadata;
mod ui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        ui::run_annotate_tui(metadata::read_annotations());
    } else {
        metadata::annotate(args.join(" "));
    }
}
