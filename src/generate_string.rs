use crate::create_structs::{XMLField, XMLStruct, PRIMITIVE_TYPES}; 
use crate::generate_xml::get_dependency_version;
use crate::string_utils::{remove_vec, to_camel_case_with_prefix, to_snake_case};

use std::collections::HashMap;

/// Generates String for Rust structs with fields and attributes. 
pub fn generate_structs_string(structs: &HashMap<String, XMLStruct>) -> String {
    let mut struct_string = String::new();
    struct_string += &format!("// Generated with schema_generator {}\n", get_dependency_version("Cargo.toml").unwrap_or("0.0.0".to_string()));
    struct_string += &format!("use serde::{{Serialize, Deserialize}};\n\n");

    for (name, xml_struct) in structs {
        let struct_name = to_camel_case_with_prefix(&name); 

        struct_string += &format!("#[derive(Serialize, Deserialize, Debug)]\n");
        struct_string += &format!("pub struct {} {{\n", struct_name);

        for field in &xml_struct.fields {

            // Check if the field type is a struct, if so, use the struct name. Otherwise, use String
            let field_type = if (*structs).contains_key(remove_vec(&field.field_type).as_str()) {
                to_camel_case_with_prefix(&field.field_type)
            } else {
                let primitive_types_guard = PRIMITIVE_TYPES.lock().unwrap(); // Lock the Mutex

                if let Some(value) = primitive_types_guard.get(&field.field_type) {
                    value.to_string()
                } else {
                    "String".to_string()
                }
            };

            struct_string = field_to_struct_string(field, &field_type, struct_string.clone());
        }

        struct_string += &format!("}}\n");
        struct_string += &format!("\n");
    }

    struct_string
}

/// Adds field to the struct string.
/// Makes all fields optional, except for attribute fields.
/// Makes attribute field "srsName" optional for GmlPolygon and GmlMultiPolygon.
pub fn field_to_struct_string(
    field: &XMLField, 
    field_type: &str, 
    mut struct_string: String
) -> String {

    // Check if the field is text
    if field.name.starts_with("$") {
        struct_string += &format!("\t#[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", field.name);
        struct_string += &format!("\tpub {}: Option<{}>,\n", field.name.replace("$", ""), field_type);
    
    // Check if the field is an attribute
    } else if field.name.starts_with("@") {

        // Check if the field is schema instance type
        if field.name.starts_with("@xsi") {
            struct_string += &format!("\t#[serde(rename = \"{}\")]\n", field.name.replace("xsi:", ""));

            let field_name = field.name.chars().skip(1).collect::<String>().replace(":", "_");
            struct_string += &format!("\tpub {}: {},\n", to_snake_case(&field_name), field_type);

        // Check if the field is a type attribute
        } else if field.name.ends_with("_type") {
            struct_string += &format!("\t#[serde(rename = \"@type\")]\n");
            struct_string += &format!("\tpub {}: {},\n", field.name.replace("@", ""), field_type);
        } else {

            // Make srsName attribute for GmlPolygon and GmlMultiPolygon optional
            if field.name.ends_with("srsName") {
                struct_string += &format!("\t#[serde(rename = \"@{}\", skip_serializing_if = \"Option::is_none\")]\n", field.name.chars().skip(1).collect::<String>());
                
                let field_name = field.name.chars().skip(1).collect::<String>().replace(":", "_");
                struct_string += &format!("\tpub {}: Option<{}>,\n", to_snake_case(&field_name), field_type);
            
            // Handle other attributes
            } else {
                struct_string += &format!("\t#[serde(rename = \"@{}\")]\n", field.name.chars().skip(1).collect::<String>());

                let field_name = field.name.chars().skip(1).collect::<String>().replace(":", "_");
                struct_string += &format!("\tpub {}: {},\n", to_snake_case(&field_name), field_type);
            }
        }

    // Handle regular fields
    } else {
        let renaming = field.name.split(":").last().unwrap();
        let field_name = field.name.split(":").next().unwrap().to_owned() + "_" + to_snake_case(&renaming).as_str();
        
        struct_string += &format!("\t#[serde(rename = \"{}\", skip_serializing_if = \"Option::is_none\")]\n", renaming);
        struct_string += &format!("\tpub {}: Option<{}>,\n", field_name, field_type);
    }

    struct_string
}