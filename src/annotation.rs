use std::fmt::{self, Display, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents an annotation with content and a timestamp.
///
/// # Fields
///
/// - `content`: The textual content of the annotation. It can be any valid string, allowing you to
///              provide context, notes etc.
///
/// - `created_at`: The timestamp when the annotation was created, measured in milliseconds since the
///                 Unix epoch. This timestamp provides a point in time reference for when the annotation
///                 was added or recorded.
///
/// # Examples
///
/// ```
/// let annotation = Annotation {
///     content: "This is an example annotation.".to_string(),
///     created_at: 1632172800000, // Timestamp in milliseconds
/// };
/// ```
pub struct Annotation {
    /// Annotation content.
    pub content: String,

    /// Created at timestamp in milliseconds.
    pub created_at: u64,
}

impl Annotation {
    /// Formats the timestamp difference between the current time and the `created_at` timestamp
    /// as a human-readable string.
    ///
    /// This function calculates the time difference between the current time and the provided
    /// `created_at` timestamp, and formats it in a human-friendly way, such as "2 minutes ago,"
    /// "1 hour ago," or "3 years ago."
    ///
    /// # Arguments
    ///
    /// - `self`: A reference to the `Annotation` instance containing the `created_at` timestamp.
    ///
    /// # Returns
    ///
    /// - A `String` representing the formatted time difference. If the timestamp is in the future,
    ///   the function will cause a panic.
    ///
    /// # Examples
    ///
    /// ```
    /// let annotation = Annotation {
    ///     content: "This is an example annotation.".to_string(),
    ///     created_at: 1632172800000, // Timestamp in milliseconds
    /// };
    ///
    /// let formatted_time = annotation.format_created_at();
    /// println!("Created: {}", formatted_time); // Prints a formatted timestamp difference.
    /// ```
    pub fn format_created_at(&self) -> String {
        let current_time = SystemTime::now();
        let timestamp = UNIX_EPOCH + std::time::Duration::from_millis(self.created_at);

        if let Ok(duration) = current_time.duration_since(timestamp) {
            if duration.as_secs() == 0 {
                return "Just now".to_string();
            }

            let seconds = duration.as_secs();

            if seconds < 60 {
                return format!("{:02} seconds ago", seconds);
            }

            let minutes = seconds / 60;

            if minutes < 60 {
                return format!("{:02} minutes ago", minutes);
            }

            let hours = minutes / 60;

            if hours < 24 {
                return format!("{:02} hours ago", hours);
            }

            let days = hours / 24;

            if days < 365 {
                return format!("{:02} days ago", days);
            }

            let years = days / 365;
            return format!("{:02} years ago", years);
        }

        panic!("Invalid timestamp!")
    }
}

/// Implements the conversion from a `&String` to an `Annotation` struct.
///
/// This conversion allows you to create an `Annotation` from a string that follows a specific
/// format used in metadata files. Annotations in the metadata file are stored in the following
/// format:
///
/// ```text
/// <TIMESTAMP> <CONTENT>
/// ```
///
/// Where:
///
/// - `<TIMESTAMP>`: Represents a valid u128 number that serves as the timestamp of the annotation.
/// - `<CONTENT>`: Represents the textual content of the annotation as a valid string.
///
/// To parse an annotation, this implementation locates the first space character in the string.
/// Since the timestamp is a contiguous sequence of characters, this space character indicates the
/// separation between the annotation's timestamp and its content.
///
/// However, it's important to note that if an annotation is ever stored in an invalid format, such
/// as when the annotation's timestamp is missing, certain issues may arise:
///
/// - The character space between the timestamp and content may not be found at all.
/// - A character space from within the annotation's content might be incorrectly identified as
///   the separator between the supposed timestamp and its content.
///
/// # Errors
///
/// If the provided string cannot be successfully parsed into an `Annotation`, this conversion
/// will panic with an error message indicating the cause of the failure.
///
/// # Examples
///
/// ```rust
/// let annotation_string = "1632172800000 This is an example annotation.".to_string();
/// let annotation: Annotation = (&annotation_string).into();
/// println!("{}", annotation);
/// ```
impl From<&String> for Annotation {
    fn from(string: &String) -> Annotation {
        // In the matadata file, annotations are stored in the following format:
        //
        //      <ANNOTATION-FILE> ::= { <ANNOTATION> \n }
        //
        // in which,
        //
        //      <ANNOTATION> ::= <TIMESTAMP> <CONTENT>
        //      <TIMESTAMP>  ::= Every valid u128 number
        //      <CONTENT>    ::= Every valid string.
        //
        // To parse an annotation, just locate the first space character. Since the
        // timestamp is a contiguous sequence of characters, this space character indicates
        // the separation between the annotation's timestamp and its content.
        //
        // However, if an annotation is ever stored in an invalid format, such as when the
        // annotation's timestamp is missing, certain issues may arise. In such cases:
        //
        // - The character space between the timestamp and content may not be found at all.
        // - A character space from within the annotation's content might be incorrectly identified
        //   as the separator between the supposed timestamp and its content.
        let created_at_delim_pos = string
            .chars()
            .position(|c| c == ' ')
            .expect("Unable to find the 'created_at' delimiter position in the string.");

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

/// Implements the `Display` trait for the `Annotation` struct.
///
/// This trait allows you to customize how an `Annotation` is formatted when it is displayed
/// as a string. In this implementation, an `Annotation` is represented as a tuple-like structure:
///
/// ```text
/// (<TIMESTAMP>, <CONTENT>)
/// ```
///
/// Where:
///
/// - `<TIMESTAMP>`: Represents the timestamp of the annotation.
/// - `<CONTENT>`: Represents the textual content of the annotation.
///
/// This format is designed for easy and human-readable representation of annotations when they
/// need to be printed, logged, or otherwise displayed as strings.
///
/// # Examples
///
/// ```rust
/// let annotation = Annotation {
///     created_at: 1632172800000,
///     content: "This is an example annotation.".to_string(),
/// };
///
/// let formatted_annotation = format!("{}", annotation);
/// println!("{}", formatted_annotation); // Prints: "(1632172800000, This is an example annotation.)"
/// ```
///
/// # Note
///
/// - This implementation is primarily intended for human-readable output and debugging purposes.
/// - The `<TIMESTAMP>` and `<CONTENT>` placeholders represent the actual timestamp and content
///   of the annotation.
impl Display for Annotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.created_at, self.content)
    }
}
