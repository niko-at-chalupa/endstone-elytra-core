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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    #[tracing_test::traced_test]
    fn kebab_stuff() {
        // unkebbabed: kebabbed
        let names = HashMap::from([
            ("endstone chat relay", "endstone-chat-relay"),
            ("endstone-chatrelay", "endstone-chatrelay"),
            ("endstoneChatRelay", "endstonechatrelay"),
            ("endstone--chat-relay---chat-", "endstone-chat-relay-chat"),
        ]);

        for name in names.iter() {
            let kebabbed_name: &str = &kebabify(&name.0);
            
            tracing::info!("{}, {}", kebabbed_name, name.1);

            assert_eq!(&kebabbed_name, name.1);
        }
    }
}