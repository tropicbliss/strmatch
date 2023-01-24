# strmatch

Conditionally match strings in Rust using regex without much boilerplate. Yes, this uses [`once_cell`](https://docs.rs/once_cell/latest/once_cell/).

# Usage

```rs
#[derive(PartialEq, Eq, Debug)]
enum StringType {
    Phone,
    Email,
    Others,
}

let email = "example@example.com";
let result = strmatch!(email => {
    r#"(\d{4})-(\d{2})-(\d{2})"# => StringType::Phone,
    r#"^([a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})*$"# => StringType::Email,
    _ => StringType::Others
});
assert_eq!(StringType::Email, result);

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
```
