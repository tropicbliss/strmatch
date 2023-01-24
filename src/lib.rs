pub use once_cell::sync::Lazy;
pub use regex::Regex;

/// Conditionally match strings in Rust using regex without much boilerplate. Yes, this uses [`once_cell`](https://docs.rs/once_cell/latest/once_cell/).
///
/// # Usage
/// ```rust
/// use strmatch::strmatch;
///
/// #[derive(PartialEq, Eq, Debug)]
/// enum StringType {
///     Phone,
///     Email,
///     Others,
/// }
///
/// let email = "example@example.com";
/// let result = strmatch!(email => {
///     r#"(\d{4})-(\d{2})-(\d{2})"# => StringType::Phone,
///     r#"^([a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})*$"# => StringType::Email,
///     _ => StringType::Others
/// });
/// assert_eq!(StringType::Email, result);
///
/// let result = strmatch!("example@example.com" => {
///     // Phone
///     r#"(\d{4})-(\d{2})-(\d{2})"# => {
///         1 + 2
///     },
///     // Email
///     r#"^([a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})*$"# => {
///         3 + 4
///     },
///     _ => 5,
/// });
/// assert_eq!(7, result);
/// ```

#[macro_export]
macro_rules! strmatch {
    ($input:expr => {$($regex:literal => $result:expr),+, _ => $default:expr}) => {'strmatch: {
        $(
            {
                static REGEX: $crate::Lazy<$crate::Regex> = $crate::Lazy::new(|| $crate::Regex::new($regex).unwrap());
                if REGEX.is_match($input) {
                    break 'strmatch ($result);
                }
            }
        )*
        break 'strmatch ($default);
    }};
    ($input:expr => {$($regex:literal => $result:expr),+, _ => $default:expr,}) => {
        strmatch!($input => {$($regex => $result),*, _ => $default})
    };
}

#[cfg(test)]
mod tests {
    #[derive(PartialEq, Eq, Debug)]
    enum StringType {
        Phone,
        Email,
        Others,
    }

    #[test]
    fn without_braces_trailing_and_var() {
        let result = strmatch!("example@example.com" => {
            r#"(\d{4})-(\d{2})-(\d{2})"# => StringType::Phone,
            r#"^([a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})*$"# => StringType::Email,
            _ => StringType::Others,
        });
        assert_eq!(StringType::Email, result);
    }

    #[test]
    fn without_braces() {
        let result = strmatch!("example@example.com" => {
            r#"(\d{4})-(\d{2})-(\d{2})"# => StringType::Phone,
            r#"^([a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})*$"# => StringType::Email,
            _ => StringType::Others
        });
        assert_eq!(StringType::Email, result);
    }

    #[test]
    fn with_braces() {
        let result = strmatch!("example@example.com" => {
            // Phone
            r#"(\d{4})-(\d{2})-(\d{2})"# => {
                1 + 2
            },
            // Email
            r#"^([a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})*$"# => {
                3 + 4
            },
            _ => 5,
        });
        assert_eq!(7, result);
    }

    #[test]
    fn variables() {
        let email = "example@example.com";
        let result = strmatch!(email => {
            r#"(\d{4})-(\d{2})-(\d{2})"# => StringType::Phone,
            r#"^([a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})*$"# => StringType::Email,
            _ => StringType::Others,
        });
        assert_eq!(StringType::Email, result);
    }
}
