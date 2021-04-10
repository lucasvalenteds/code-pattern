struct Pattern {
    mask: String,
    delimiter: String,
}

fn pretty_print(raw_code: String, pattern: Pattern) -> Option<String> {
    let max_code_size = pattern.mask.replace(&pattern.delimiter, "");
    if raw_code.len() > max_code_size.len() {
        return None;
    }

    let mut cursor = 0;
    let mut code: Vec<String> = Vec::new();

    let placeholders = pattern.mask.split(&pattern.delimiter);
    for placeholder in placeholders.into_iter() {
        let characters_required = placeholder.len();

        let slice: String = raw_code
            .chars()
            .skip(cursor)
            .take(characters_required)
            .collect();

        if slice.len() == characters_required {
            code.push(slice);
            cursor += characters_required;
        } else if slice.len() == 0 {
            break;
        } else {
            code.clear();
            break;
        }
    }

    match code.len() {
        0 => None,
        _ => Some(code.join(&pattern.delimiter)),
    }
}

fn main() {}

#[cfg(test)]
mod tests {

    use super::{pretty_print, Pattern};
    use rstest::*;

    #[rstest]
    #[case("1", "1")]
    #[case("123", "1;23")]
    #[case("123456", "1;23;456")]
    fn test_valid_code(#[case] raw_code: &str, #[case] expected_code: &str) {
        let code = pretty_print(
            raw_code.to_string(),
            Pattern {
                mask: String::from("x;xx;xxx"),
                delimiter: String::from(";"),
            },
        );

        assert_eq!(Some(expected_code.to_string()), code)
    }

    #[rstest]
    #[case("12")]
    #[case("1234")]
    #[case("12345")]
    fn test_invalid_code(#[case] raw_code: &str) {
        let code = pretty_print(
            raw_code.to_string(),
            Pattern {
                mask: String::from("x;xx;xxx"),
                delimiter: String::from(";"),
            },
        );

        assert_eq!(None, code)
    }
}
