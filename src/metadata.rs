use crate::annotation::Annotation;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const ANNOTATIONS_FILENAME: &str = ".annotations";

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
/// ```
/// let content = "This is a new annotation.";
/// annotate(content.to_string());
/// ```
///
/// # Note
///
/// - This function is designed to add new annotations to the metadata file in a specific format, where each line
///   represents an annotation entry. The format is as follows:
///
///   ```
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
pub fn annotate(content: String) {
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
/// ```
/// let annotations = read_annotations();
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
///   ```
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
pub fn read_annotations() -> Vec<Annotation> {
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

/// Reads and collects lines from an annotations file.
///
/// This function is responsible for opening and reading an annotations file, collecting its lines,
/// and returning them as a `Result<Vec<String>, io::Error>`. If the file is not found or cannot
/// be opened, the function will return an error.
///
/// # Returns
///
/// - A `Result` containing either a `Vec<String>` with lines from the file or an `io::Error` in case
///   of file I/O issues.
///
/// # Examples
///
/// ```
/// match read_annotations_file() {
///     Ok(lines) => {
///         for line in lines {
///             println!("{}", line);
///         }
///     }
///     Err(e) => {
///         eprintln!("Error reading annotations file: {}", e);
///     }
/// }
/// ```
///
/// # Note
///
/// - This function is designed to read and collect lines from an annotations file. It returns the lines
///   as a `Result<Vec<String>, io::Error>`.
///
/// - If the annotations file is not found, it will create an empty file and return an empty `Vec<String>`.
pub fn read_annotations_file() -> Result<Vec<String>, io::Error> {
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
