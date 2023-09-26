use crate::annotation::Annotation;

use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{Dialog, LinearLayout, SelectView, TextView};
use cursive::Cursive;

/// Creates a well-formatted string suitable for presentation within the annotation layout.
///
/// This function creates a [`String`] that represents the formatted text of an [`Annotation`].
/// The string consists of two elements:
///
/// 1. A timestamp indicating when the annotation was created, formatted as a human-readable string,
///    followed by a separator.
///
/// 2. The textual content of the annotation.
///
/// # Arguments
///
/// - `annotation`: A reference to the [`Annotation`] instance that contains the timestamp and content
///   to be displayed.
///
/// # Returns
///
/// - A [`String`] containing the formatted text elements.
///
/// # Examples
///
/// ```rust
/// let annotation = Annotation {
///     content: "This is an example annotation.".to_string(),
///     created_at: 1632172800000, // Timestamp in milliseconds
/// };
///
/// let annotation_text = build_annotation_text(&annotation);
/// ```
pub fn build_annotation_text(annotation: &Annotation) -> String {
    format!(
        "{:>14} | {}",
        annotation.format_created_at(),
        annotation.content
    )
}

fn on_submit_annotation(s: &mut Cursive, _content: &String) {
    let dialog = Dialog::new()
        .title("Annotation")
        .button("Ok", |s| {
            // Close the dialog.
            s.pop_layer();
        })
        .button("Remove", |s| {
            s.call_on_name("annotation_list", |view: &mut SelectView| {
                // Ensure an item is selected before removing it.
                if let Some(selected_id) = view.selected_id() {
                    view.remove_item(selected_id);
                }
            });

            // Close the dialog.
            s.pop_layer();
        });

    s.add_layer(dialog);
}

/// Builds the annotation layout for displaying a list of annotations.
///
/// This function creates a dialog containing a scrollable view that presents a list of annotations
/// in a user-friendly format. The layout includes a message for when there are no annotations, and
/// for each annotation in the list, it includes a formatted text layout created using the
/// [`build_annotation_text`] function.
///
/// # Arguments
///
/// - `annotations`: A reference to a slice of [`Annotation`] instances representing the list of
///   annotations to be displayed.
///
/// # Returns
///
/// - A [`Dialog`] containing a `ScrollView<LinearLayout>` which further contains the formatted
///   annotations and optional messages.
///
/// # Examples
///
/// ```rust
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
pub fn build_annotations_layout(annotations: &[Annotation]) -> Dialog {
    if annotations.is_empty() {
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("You have not registered any annotation!"))
                .child(TextView::new("Try: jotspot [text]").center())
                .scrollable(),
        )
        .title("Annotations")
    } else {
        let select_view = annotations
            .iter()
            .fold(SelectView::new(), |select_view, annotation| {
                select_view.item_str(build_annotation_text(annotation))
            })
            .on_submit(on_submit_annotation)
            .with_name("annotation_list");

        Dialog::around(select_view.scrollable()).title("Annotations")
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
/// - `annotations`: A vector of [`Annotation`] instances representing the list of annotations to be displayed
///   and interacted with in the TUI.
///
/// # Examples
///
/// ```rust
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

    siv.add_layer(build_annotations_layout(&annotations));
    siv.run();
}

