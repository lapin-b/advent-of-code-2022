use std::borrow::Cow;


pub fn get_file_from_argv<'a>(default: &'a str) -> Cow<'a, str> {
    std::env::args()
        .nth(1)
        .map(|s| Cow::from(s))
        .unwrap_or_else(|| {
            println!("Warning: No file passed, using default {}", default);
            Cow::from(default)
        })
}