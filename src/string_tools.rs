

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

#[cfg(test)]
mod tests {
    use super::camel_case_to_snake_case;

    #[test]
    fn test_camel_case_to_snake_case() {
        assert_eq!(camel_case_to_snake_case("serializedVersion"), "serialized_version");
        assert_eq!(camel_case_to_snake_case("m_ObjectHideFlags"), "object_hide_flags");
        assert_eq!(camel_case_to_snake_case("m_CorrespondingSourceObject"), "corresponding_source_object");
        assert_eq!(camel_case_to_snake_case("m_PrefabInstance"), "prefab_instance");
        assert_eq!(camel_case_to_snake_case("stringTagMap"), "string_tag_map");
        assert_eq!(camel_case_to_snake_case("disabledShaderPasses"), "disabled_shader_passes");
    }
}