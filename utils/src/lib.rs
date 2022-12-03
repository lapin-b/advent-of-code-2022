use std::borrow::Cow;


pub fn get_file_from_argv(default: &str) -> Cow<'_, str> {
    std::env::args()
        .nth(1)
        .map(Cow::from)
        .unwrap_or_else(|| {
            println!("Warning: No file passed, using default {}", default);
            Cow::from(default)
        })
}