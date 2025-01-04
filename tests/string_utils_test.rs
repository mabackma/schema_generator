use schema_generator::string_utils::{to_camel_case_with_prefix, to_snake_case};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case_with_prefix_basic() {
        let input = "re:AreaNumber";
        let expected = "ReAreaNumber";
        let result = to_camel_case_with_prefix(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_camel_case_with_prefix_no_prefix() {
        let input = "AreaNumber";
        let expected = "AreaNumber";
        let result = to_camel_case_with_prefix(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_snake_case_basic() {
        assert_eq!(to_snake_case("AreaNumber"), "area_number");
    }

    #[test]
    fn test_to_snake_case_already_snake_case() {
        assert_eq!(to_snake_case("area_number"), "area_number");
    }

    #[test]
    fn test_to_snake_case_single_word() {
        assert_eq!(to_snake_case("word"), "word");
    }

    #[test]
    fn test_to_snake_case_empty_string() {
        assert_eq!(to_snake_case(""), "");
    }
}