// StringReplace

/// SlashAdd
fn add_slashes(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '\'' => result.push_str("\\'"),
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\0' => result.push_str("\\0"),
            _ => result.push(c),
        }
    }
    result
}
// SlashDel
fn del_slashes(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek() == Some(&'\\') {
            result.push(c);
            chars.next();
        } else if c == '\\' && chars.peek() == Some(&'\'') {
            result.push('\'');
            chars.next();
        } else if c == '\\' && chars.peek() == Some(&'"') {
            result.push('"');
            chars.next();
        } else {
            result.push(c);
        }
    }

    result
}

// CamelToSnake
fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    let mut prev_upper = false;

    for c in s.chars() {
        if c.is_ascii_uppercase() {
            if !prev_upper {
                result.push('_');
                prev_upper = true;
            }
            result.push(c.to_ascii_lowercase());
        } else {
            prev_upper = false;
            result.push(c);
        }
    }

    result
}
// SnakeToCamel
fn snake_to_camel(s: &str) -> String {
    let mut result = String::new();
    let mut is_uppercase = false;

    for c in s.chars() {
        if c == '_' {
            is_uppercase = true;
        } else if is_uppercase {
            result.push(c.to_ascii_uppercase());
            is_uppercase = false;
        } else {
            result.push(c);
        }
    }

    result
}

// remove duplicate
pub fn remove_duplicates(input: &str, delimiter: &str) -> String {
    let mut exists = Vec::new();
    let mut result = String::new();

    for num_str in input.split(delimiter) {
        if num_str.chars().all(|c| c.is_numeric()) {
            if let Ok(num) = num_str.parse::<i32>() {
                if !exists.contains(&num) {
                    exists.push(num);
                    result.push_str(&num.to_string());
                    result.push_str(delimiter);
                }
            }
        }
    }
    if !result.is_empty() {
        result.trim_end_matches(delimiter).to_owned()
    } else {
        result
    }
}

fn vec_u8_to_string(vec: Vec<u8>) -> Option<String> {
    match String::from_utf8(vec) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}
