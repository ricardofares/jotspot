use crate::annotation::Annotation;

use cursive::view::Scrollable;
use cursive::views::{Dialog, LinearLayout, ScrollView, TextView};

/// Builds a text layout for displaying an `Annotation`.
///
/// This function creates a `LinearLayout` that represents the formatted text of an `Annotation`.
/// The layout consists of two elements:
///
/// 1. A timestamp indicating when the annotation was created, formatted as a human-readable string,
///    followed by a separator.
///
/// 2. The textual content of the annotation.
///
/// # Arguments
///
/// - `annotation`: A reference to the `Annotation` instance that contains the timestamp and content
///   to be displayed.
///
/// # Returns
///
/// - A `LinearLayout` containing the formatted text elements.
///
/// # Examples
///
/// ```
/// let annotation = Annotation {
///     content: "This is an example annotation.".to_string(),
///     created_at: 1632172800000, // Timestamp in milliseconds
/// };
///
/// let annotation_layout = build_annotation_text(&annotation);
/// ```
pub fn build_annotation_text(annotation: &Annotation) -> LinearLayout {
    LinearLayout::horizontal()
        .child(TextView::new(format!("{:>14} | ", annotation.format_created_at())))
        .child(TextView::new(annotation.content.clone()))
}

/// Builds a scrollable layout for displaying a list of annotations.
///
/// This function creates a scrollable view that presents a list of annotations in a user-friendly
/// format. The layout includes a message for when there are no annotations, and for each annotation
/// in the list, it includes a formatted text layout created using the `build_annotation_text` function.
///
/// # Arguments
///
/// - `annotations`: A reference to a slice of `Annotation` instances representing the list of
///   annotations to be displayed.
///
/// # Returns
///
/// - A `ScrollView<LinearLayout>` containing the formatted annotations and optional messages.
///
/// # Examples
///
/// ```
/// let annotations: Vec<Annotation> = vec![
///     Annotation {
///         content: "This is an example annotation 1.".to_string(),
///         created_at: 1632172800000,
///     },
///     Annotation {
///         content: "This is an example annotation 2.".to_string(),
///         created_at: 1632172900000,
///     },
/// ];
///
/// let annotations_layout = build_annotations_layout(&annotations);
/// ```
pub fn build_annotations_layout(annotations: &[Annotation]) -> ScrollView<LinearLayout> {
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

/// Runs the Text User Interface (TUI) for annotating and displaying a list of annotations.
///
/// This function initializes and runs a Cursive-based Text User Interface (TUI) to interactively
/// annotate and display a list of annotations. It creates a user-friendly interface that includes
/// a list of annotations with timestamps and content.
///
/// # Arguments
///
/// - `annotations`: A vector of `Annotation` instances representing the list of annotations to be displayed
///   and interacted with in the TUI.
///
/// # Examples
///
/// ```
/// let annotations: Vec<Annotation> = vec![
///     Annotation {
///         content: "This is an example annotation 1.".to_string(),
///         created_at: 1632172800000,
///     },
///     Annotation {
///         content: "This is an example annotation 2.".to_string(),
///         created_at: 1632172900000,
///     },
/// ];
///
/// run_annotate_tui(annotations);
/// ```
pub fn run_annotate_tui(annotations: Vec<Annotation>) {
    let mut siv = cursive::default();

    siv.add_layer(Dialog::around(build_annotations_layout(&annotations)).title("Annotations"));
    siv.run();
}
