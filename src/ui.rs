use crate::annotation::{Annotation, AnnotationsData};
use crate::metadata;

use cursive::theme::{Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::view::{Nameable, Scrollable};
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

/// Handles the submission of an annotation.
///
/// This function is called when a user submits an annotation. It displays a dialog box with
/// options to either remove the submitted annotation or keep it. If the user chooses to remove
/// the annotation, it is removed from the list and the metadata is updated accordingly.
///
/// # Arguments
///
/// - `s`: A mutable reference to the [`Cursive`] instance.
/// - `_content`: A reference to the content of the submitted annotation (not used in this function).
fn on_submit_annotation(s: &mut Cursive, _content: &String) {
    let dialog = Dialog::text("Would you like to remove the annotation?")
        .title("Annotation")
        .button("No", |s| {
            // Close the dialog.
            s.pop_layer();
        })
        .button("Yes", |s| {
            let mut select_view = s
                .find_name::<SelectView>("annotation_list")
                .expect("It there must be a select view named `annotation_list`");

            let data = s
                .user_data::<AnnotationsData>()
                .expect("It there must be a user data");

            if let Some(selected_id) = select_view.selected_id() {
                select_view.remove_item(selected_id);
                data.get_annotations_mut().remove(selected_id);
            }

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
    let annotations_layout = build_annotations_layout(&annotations);

    siv.set_user_data(AnnotationsData::new(annotations));
    siv.add_layer(annotations_layout);
    siv.run();

    metadata::save_annotations(siv.user_data().expect("It there must be a data"));
}
