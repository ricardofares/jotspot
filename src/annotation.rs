use chrono::{Local, TimeZone, Utc};
use std::fmt::{self, Display, Formatter};

/// Represents an annotation with content and a timestamp.
///
/// This struct represents an annotation with a textual content and a timestamp indicating when the
/// annotation was created. The timestamp is measured in milliseconds since the Unix epoch.
pub struct Annotation {
    /// The textual content of the annotation.
    pub content: String,

    /// The timestamp when the annotation was created, measured in milliseconds since the Unix epoch.
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
    /// ```rust
    /// let annotation = Annotation {
    ///     content: "This is an example annotation.".to_string(),
    ///     created_at: 1632172800000, // Timestamp in milliseconds
    /// };
    ///
    /// let formatted_time = annotation.format_created_at();
    /// println!("Created: {}", formatted_time); // Prints a formatted timestamp difference.
    /// ```
    pub fn format_created_at(&self) -> String {
        let timestamp = Utc
            .timestamp_millis_opt(self.created_at as i64)
            .unwrap()
            .with_timezone(&Local);
        let now = Local::now().naive_local();
        let duration = now.signed_duration_since(timestamp.naive_local());

        if duration.num_seconds() == 0 {
            return "Just now".to_string();
        }

        if duration.num_seconds() < 60 {
            return format!("{} seconds ago", duration.num_seconds());
        }

        let minutes = duration.num_minutes();

        if minutes < 60 {
            return format!("{} minutes ago", minutes);
        }

        let hours = duration.num_hours();

        if hours < 24 {
            return format!("{} hours ago", hours);
        }

        let days = duration.num_days();

        if days < 365 {
            return format!("{} days ago", days);
        }

        let years = days / 365;
        return format!("{} years ago", years);
    }
}

impl From<&str> for Annotation {
    fn from(string: &str) -> Annotation {
        let created_at_delim_pos = string
            .find(' ')
            .expect("Unable to find the 'created_at' delimiter position in the string.");

        let (created_at_str, content_str) = string.split_at(created_at_delim_pos);
        let content_str = &content_str[1..]; // Skip the space

        let created_at = created_at_str
            .parse()
            .expect("From<&str> for Annotation: created_at could not be parsed.");

        Annotation {
            content: content_str.to_string(),
            created_at,
        }
    }
}

impl Display for Annotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.created_at, self.content)
    }
}
