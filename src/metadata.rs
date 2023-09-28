use crate::annotation::{Annotation, AnnotationsData};

use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Get the annotations metadata file path.
///
/// This function retrieves the home directory path of the current user and appends the
/// ".annotations" file name to it, forming the complete file path for storing annotations.
///
/// # Returns
///
/// A `PathBuf` containing the complete file path for the annotations metadata file, including the
/// home directory and the filename.
///
/// # Panics
///
/// This function may panic if it fails to retrieve the home directory path. In a typical
/// Unix-like environment, the "HOME" environment variable is expected to be set.
///
/// # Examples
///
/// ```rust
/// let filename = get_annotations_filename();
/// println!("Annotations file path: {}", filename.display());
/// ```
///
/// # Note
///
/// - This function is designed to provide a standardized file path for the annotations file,
///   assuming that it should be stored in the user's home directory with the filename ".annotations".
pub fn get_annotations_filename() -> PathBuf {
    let homedir_path = env::var("HOME").expect("Failed to get the home directory");
    PathBuf::from(homedir_path).join(".annotations")
}

/// Appends a new annotation to the metadata file with a timestamp and content.
///
/// This function is used to add a new annotation to the metadata, including a timestamp indicating when
/// the annotation was created and the textual content of the annotation. The annotations are stored
/// in a specific format where each line represents an annotation entry.
///
/// # Arguments
///
/// - `content`: A `String` containing the textual content of the annotation to be added.
///
/// # Examples
///
/// ```rust
/// let content = "This is a new annotation.";
/// annotate(&content);
/// ```
///
/// # Note
///
/// - This function is designed to add new annotations to the metadata file in a specific format, where each line
///   represents an annotation entry. The format is as follows:
///
///   ```text
///   <TIMESTAMP> <CONTENT>
///   ```
///
///   where:
///
///   - `<TIMESTAMP>` is the timestamp in milliseconds indicating when the annotation was created.
///   - `<CONTENT>` is the textual content of the annotation.
///
/// - The `annotate` function appends the new annotation to the file, ensuring that it adheres to the
///   specified format.
///
/// - If the operation fails (e.g., due to file I/O issues), an error message is printed to the
///   standard error stream.
pub fn annotate(content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_annotations_filename())?;

    let annotation = Annotation::new(content);

    writeln!(file, "{} {}", annotation.created_at, annotation.content)?;

    Ok(())
}

/// Reads and parses annotations from the metadata file into a vector of `Annotation` instances.
///
/// This function is responsible for reading and parsing annotations from a file and converting
/// them into a vector of `Annotation` instances. Annotations in the file should be stored in a
/// specific format, with each line representing an annotation entry containing a timestamp and
/// content.
///
/// # Returns
///
/// - A `Vec<Annotation>` containing parsed annotations.
///
/// # Panics
///
/// - If the annotations file cannot be read or if any annotation entry is in an invalid format,
///   the function will panic with an error message.
///
/// # Examples
///
/// ```rust
/// let annotations = read_annotations().expect("Failed to read annotations");
/// for annotation in &annotations {
///     println!("Timestamp: {}, Content: {}", annotation.created_at, annotation.content);
/// }
/// ```
///
/// # Note
///
/// - This function is designed to read and parse annotations from a file where each line follows
///   the specified format:
///
///   ```text
///   <TIMESTAMP> <CONTENT>
///   ```
///
///   where:
///
///   - `<TIMESTAMP>` is the timestamp in milliseconds indicating when the annotation was created.
///   - `<CONTENT>` is the textual content of the annotation.
///
/// - If the annotations file is not found, empty, or contains entries in an invalid format, the function
///   will panic with an error message.
pub fn read_annotations() -> io::Result<Vec<Annotation>> {
    let mut lines = String::from("");

    OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(get_annotations_filename())?
        .read_to_string(&mut lines)?;

    let annotations = lines
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Annotation::from(line))
        .collect();

    Ok(annotations)
}

/// Saves a collection of annotations to a file.
///
/// # Arguments
///
/// - `data`: A reference to an [`AnnotationsData`] struct containing the annotations to be saved.
///
/// # Panics
///
/// This function may panic if it encounters errors while opening or writing to the file.
pub fn save_annotations(data: &AnnotationsData) {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(get_annotations_filename())
        .expect("Couldn't open the file");

    let mut content = String::new();

    for annotation in data.get_annotations() {
        content.push_str(format!("{} {}\n", annotation.created_at, annotation.content).as_str());
    }

    writeln!(file, "{}", content).expect("Couldn't write to the file");
}
