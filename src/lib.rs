#![doc = include_str!("../README.md")]
#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

#[macro_export]
/// Embeds the file contents as a string into the binary in release mode, but
/// loads it from fs in debug.
///
/// The file path should be relative to the source file from which the macro is
/// called (similar to [`include_str`]).
///
/// The macro returns a `Cow<'static, str>`, which can either be a reference to
/// the static string included in the binary, or an owned string read from the
/// file in debug mode.
///
/// Note: this macro will panic in debug mode if the file cannot be read (which
/// is what you probably want).
///
/// ## Example
///
/// ```no_run
/// # use embed_file::embed_string;
/// let my_file_contents = embed_string!("path/to/my_file.txt");
/// println!("{}", my_file_contents);
/// ```
macro_rules! embed_string {
    ($path:literal) => {{
        #[cfg(debug_assertions)]
        {
            use std::{borrow::Cow, fs, path::Path};

            let this_file = Path::new(file!());
            let this_dir = this_file
                .parent()
                .expect(&format!("embed_file: no parent for `{:?}`", this_file));
            let path = this_dir.join($path);
            let data = match fs::read_to_string(&path) {
                Ok(s) => s,
                Err(e) => panic!("embed_file: can't read `{:?}` because of {:?}", path, e),
            };
            let s: Cow<'static, str> = Cow::Owned(data);
            s
        }
        #[cfg(not(debug_assertions))]
        {
            use std::borrow::Cow;

            let s: Cow<'static, str> = Cow::Borrowed(include_str!($path));
            s
        }
    }};
}

#[macro_export]
/// Embeds the file contents as bytes into the binary in release mode, but
/// loads it from the filesystem in debug.
///
/// The file path should be relative to the source file from which the macro is
/// called (similar to [`include_bytes`]).
///
/// The macro returns a `Cow<'static, [u8]>`, which can either be a reference to
/// the static byte slice included in the binary, or an owned byte vector read
/// from the file in debug mode.
///
/// Note: this macro will panic in debug mode if the file cannot be read (which
/// is what you probably want).
///
/// ## Example
///
/// ```no_run
/// # use embed_file::embed_bytes;
/// let my_file_bytes = embed_bytes!("path/to/my_file.bin");
/// println!("File size: {}", my_file_bytes.len());
/// ```
macro_rules! embed_bytes {
    ($path:literal) => {{
        #[cfg(debug_assertions)]
        {
            use std::{borrow::Cow, fs, path::Path};

            let this_file = Path::new(file!());
            let this_dir = this_file
                .parent()
                .expect(&format!("embed_file: no parent for `{:?}`", this_file));
            let path = this_dir.join($path);
            let data = match fs::read(&path) {
                Ok(bytes) => bytes,
                Err(e) => panic!("embed_file: can't read `{:?}` because of {:?}", path, e),
            };
            let bytes: Cow<'static, [u8]> = Cow::Owned(data);
            bytes
        }
        #[cfg(not(debug_assertions))]
        {
            use std::borrow::Cow;

            let bytes: Cow<'static, [u8]> = Cow::Borrowed(include_bytes!($path));
            bytes
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_string() {
        assert!(embed_string!("lib.rs").contains("# embed-file"));
    }

    #[test]
    fn test_embed_bytes() {
        assert!(String::from_utf8(embed_bytes!("lib.rs").to_vec())
            .unwrap()
            .contains("# embed-file"));
    }
}
