use crate::create_structs::{XMLField, XMLStruct}; 
use crate::string_utils::{remove_vec, to_camel_case_with_prefix, to_snake_case};

use std::collections::HashMap;

// Generates String for Rust structs with fields 
pub fn generate_structs_string(structs: &HashMap<String, XMLStruct>) -> String {
    let mut struct_string = String::new();

    for (name, xml_struct) in structs {
        let struct_name = to_camel_case_with_prefix(&name); 
        struct_string += &format!("#[derive(Serialize, Deserialize)]\n");
        struct_string += &format!("pub struct {} {{\n", struct_name);

        for field in &xml_struct.fields {

            // Check if the field type is a struct, if so, use the struct name. Otherwise, use String
            let field_type = if (*structs).contains_key(remove_vec(&field.field_type).as_str()) {
                to_camel_case_with_prefix(&field.field_type)
            } else {
                "String".to_string()
            };

            struct_string = field_to_struct_string(field, &field_type, struct_string.clone());
        }

        struct_string += &format!("}}\n");
        struct_string += &format!("\n");
    }

    struct_string
}

// Adds field to the struct string
fn field_to_struct_string(field: &XMLField, field_type: &str, mut struct_string: String) -> String {
    // Check if the field is text
    if field.name.starts_with("$") {
        struct_string += &format!("\t#[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", field.name);
        struct_string += &format!("\tpub {}: Option<{}>,\n", field.name.replace("$", ""), field_type);
    } else if field.name.starts_with("@") {
        // Check if the field is a type attribute
        if field.name.ends_with("_type") {
            struct_string += &format!("\t#[serde(rename = \"@type\")]\n");
            struct_string += &format!("\tpub {}: {},\n", field.name.replace("@", ""), field_type);
        } else {
            // Handle other attributes
            struct_string += &format!("\t#[serde(rename = \"@{}\")]\n", field.name.chars().skip(1).collect::<String>());

            let field_name = field.name.chars().skip(1).collect::<String>().replace(":", "_");
            struct_string += &format!("\tpub {}: {},\n", to_snake_case(&field_name), field_type);
        }
    } else {
        // Handle regular fields
        let renaming = field.name.split(":").last().unwrap();
        let field_name = field.name.split(":").next().unwrap().to_owned() + "_" + to_snake_case(&renaming).as_str();
        
        struct_string += &format!("\t#[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", renaming);
        struct_string += &format!("\tpub {}: Option<{}>,\n", field_name, field_type);
    }

    struct_string
}