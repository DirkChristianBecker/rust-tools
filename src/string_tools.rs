
/// Turns a name in camel case to snake case.
/// Removes a leading m_ from the name if it encounters it.
/// m_ is often used to mark fields as members. However, this 
/// terminology does not fit to rust. 
pub fn camel_case_to_snake_case(input : &str) -> String {
    let mut r = String::from("");
    let mut tmp = input.to_string();
    if tmp.starts_with("m_") {
        tmp.remove(0);
        tmp.remove(0);
    }

    let mut first = true;
    for c in tmp.chars() {
        if c.is_uppercase() && !first {
            r.push('_');
            r.push(c.to_ascii_lowercase());

        } else if c.is_ascii_uppercase() && first {
            r.push(c.to_ascii_lowercase());
        } else {
            r.push(c);
        }

        first = false;

    }

    r    
}

/// Capitalizes the first letter of the given string.
/// If the given string is empty an empty string will be returned.
/// If the first letter of the string is already capitalized this 
/// method does nothing.
pub fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_camel_case_to_snake_case() {
        assert_eq!(camel_case_to_snake_case("serializedVersion"), "serialized_version");
        assert_eq!(camel_case_to_snake_case("m_ObjectHideFlags"), "object_hide_flags");
        assert_eq!(camel_case_to_snake_case("m_CorrespondingSourceObject"), "corresponding_source_object");
        assert_eq!(camel_case_to_snake_case("m_PrefabInstance"), "prefab_instance");
        assert_eq!(camel_case_to_snake_case("stringTagMap"), "string_tag_map");
        assert_eq!(camel_case_to_snake_case("disabledShaderPasses"), "disabled_shader_passes");
    }

    #[test]
    fn test_capitalize_first_letter() {
        let mut tests : HashMap<&str, &str> = HashMap::new();
        tests.insert("test", "Test"); 
        tests.insert("Test", "Test");
        tests.insert("1test", "1test");
        tests.insert(" test", " test");
        tests.insert("_test", "_test");

        for (t, r) in tests {
            assert_eq!(capitalize_first_letter(t), r);
        }
    }
}