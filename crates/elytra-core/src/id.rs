use std::hash::Hasher;
use twox_hash::XxHash64;

// This is supposed to be "HASH_SEED," but I accidentally typed
// "NASH_SEED" instead. Nash is somebody I know, and I giggled
// when I noticed so it's kept in.
const NASH_SEED: u64 = u64::from_be_bytes(*b"elytra!\0");

/// Generates a plugin ID that derives from the kebab-case name
/// of plugins.
pub fn compute_plugin_id(kebab_name: &str) -> u64 {
    let mut hasher = XxHash64::with_seed(NASH_SEED);
    hasher.write(kebab_name.as_bytes());
    hasher.finish()
}

/// Converts a display name into a kebabbed name.
///
/// Kebabbed names should be unique per plugin and they determine the ID.
/// Please set them as UNIQUE in the DB.
pub fn kebabify(name: &str) -> Box<str> {
    let mut slug = String::with_capacity(name.len());
    let mut last_was_hyphen = false;

    for c in name.chars() {
        if c.is_alphanumeric() {
            for lowercase_char in c.to_lowercase() {
                slug.push(lowercase_char);
            }
            last_was_hyphen = false;
        } else if c.is_whitespace() || c == '-' || c == '_' {
            if !slug.is_empty() && !last_was_hyphen {
                slug.push('-');
                last_was_hyphen = true;
            }
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    slug.into_boxed_str()
}