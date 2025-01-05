use schema_generator::generate_string::field_to_struct_string;
use schema_generator::create_structs::XMLField;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create XMLField for testing
    fn create_xml_field(name: &str) -> XMLField {
        XMLField {
            name: name.to_string(),
            field_type: "String".to_string(),
        }
    }

    #[test]
    fn test_field_to_struct_string_text_field() {
        let field = create_xml_field("$text");
        let result = field_to_struct_string(&field, "String", String::new());

        let expected = "\t#[serde(rename = \"$text\", skip_serializing_if = \"Option::is_none\")]\n\tpub text: Option<String>,\n";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_field_to_struct_string_attribute_field() {
        let field = create_xml_field("@xsi:something");
        let result = field_to_struct_string(&field, "String", String::new());

        let expected = "\t#[serde(rename = \"@something\")]\n\tpub xsi_something: String,\n";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_field_to_struct_string_srsname_attribute_field() {
        let field = create_xml_field("@srsName");
        let result = field_to_struct_string(&field, "String", String::new());

        let expected = "\t#[serde(rename = \"@srsName\", skip_serializing_if = \"Option::is_none\")]\n\tpub srs_name: Option<String>,\n";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_field_to_struct_string_regular_field() {
        let field = create_xml_field("namespace:fieldName");
        let result = field_to_struct_string(&field, "String", String::new());

        let expected = "\t#[serde(rename = \"fieldName\", skip_serializing_if = \"Option::is_none\")]\n\tpub namespace_field_name: Option<String>,\n";

        assert_eq!(result, expected);
    }

    #[test]
    fn test_field_to_struct_string_field_with_type() {
        let field = create_xml_field("@field_with_type");
        let result = field_to_struct_string(&field, "String", String::new());

        let expected = "\t#[serde(rename = \"@type\")]\n\tpub field_with_type: String,\n";

        assert_eq!(result, expected);
    }
}
