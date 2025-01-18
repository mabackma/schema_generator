use crate::generate_string::generate_structs_string;
use crate::string_utils::{to_camel_case_with_prefix, to_snake_case};

use std::collections::HashMap;
use quick_xml::events::Event::{Start, Empty, End, Eof};
use quick_xml::reader::Reader;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct XMLField {
    pub name: String,
    pub field_type: String,
}

#[derive(Debug)]
pub struct XMLStruct {
    pub name: String,
    pub fields: Vec<XMLField>,
}

impl Clone for XMLStruct {
    fn clone(&self) -> Self {
        XMLStruct {
            name: self.name.clone(),
            fields: self.fields.clone(),
        }
    }
}

/// Parses an XML string and generates a set of Rust structs representing the XML elements and their attributes.
///
/// Each struct corresponds to an XML element, with fields representing its attributes,
/// child elements, or text content. The field type is set to the name of the child element or String when there are no child elements.
///
/// # Parameters
/// - `xml_string`: A string slice containing the XML document to parse.
///
/// # Returns
/// - `String`: A string containing the Rust structs generated from the XML document.
///
/// # Structs
/// - **`XMLStruct`**
///   - `name: String` - The name of the XML element.
///   - `fields: Vec<XMLField>` - A list of fields within the struct.
///
/// - **`XMLField`**
///   - `name: String` - The name of the field.
///   - `field_type: String` - The type of the field (e.g., `String`, `Vec<T>`).
///
/// # Behavior
/// - Parses XML start tags (`<tag>`), self-closing tags (`<tag/>`), and end tags (`</tag>`).
/// - Converts attributes into fields prefixed with `@` (e.g., `@id` for an attribute `id`).
/// - Represents text content using a field named `$text`.
/// - Handles nested elements and maintains relationships between parent and child structs.
/// - Updates the field type to `Vec<T>` if an element occurs multiple times within its parent.
///
/// - **Text Fields (`$text`)**:
///   - Prefixed with `$` in the field name.
///   - Represented as `Option<String>` and marked as `#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]`.
///
/// - **Attributes (`@name`)**:
///   - Attributes are prefixed with `@`.
///   - Fields use the attribute's name (e.g., `@id` becomes `pub id: String`).
///   - Special cases:
///     - `@xsi:type` attributes are renamed to `type` using `#[serde(rename = "type")]`.
///     - `@type` attributes are supported with specific renaming.
///     - `@srsName` attributes are marked as optional with `Option<String>`.
///
/// - **Regular Fields**:
///   - Child elements are represented as optional fields (e.g., `pub child_name: Option<ChildType>`).
///   - Names are converted to snake_case with prefixes to handle XML namespaces (e.g., `ns:tag` becomes `ns_tag`).
/// 
/// # Example
///
/// ```rust
/// use schema_generator::create_structs::create_structs;
/// 
/// let xml_data = r#"
/// <Library xmlns="http://example.com/library" xmlns:bo="http://example.com/library/book">
///     <bo:Book id="1" author="Author One">
///         <bo:Title>Book One</bo:Title>
///         <bo:Genre>Fiction</bo:Genre>
///     </bo:Book>
///     <bo:Book id="2" author="Author Two">
///         <bo:Title>Book Two</bo:Title>
///         <bo:Genre>Non-Fiction</bo:Genre>
///     </bo:Book>
/// </Library>
/// "#;
///
/// let structs_string = create_structs(xml_data);
///
/// println!("{}", structs_string);
/// ```
///
/// Expected output:
/// ```text
/// // Generated with schema_generator Generated with schema_generator 0.1.0
/// use serde::{Serialize, Deserialize};
/// 
/// #[derive(Serialize, Deserialize, Debug)]
/// pub struct Library {
///         #[serde(rename = "@xmlns")]
///         pub xmlns: String,
///         #[serde(rename = "@xmlns:bo")]
///         pub xmlns_bo: String,
///         #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
///         pub text: Option<String>,
///         #[serde(rename = "Book", skip_serializing_if = "Option::is_none")]
///         pub bo_book: Option<Vec<BoBook>>,
/// }
/// 
/// #[derive(Serialize, Deserialize, Debug)]
/// pub struct BoBook {
///         #[serde(rename = "@id")]
///         pub id: String,
///         #[serde(rename = "@author")]
///         pub author: String,
///         #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
///         pub text: Option<String>,
///         #[serde(rename = "Title", skip_serializing_if = "Option::is_none")]
///         pub bo_title: Option<String>,
///         #[serde(rename = "Genre", skip_serializing_if = "Option::is_none")]
///         pub bo_genre: Option<String>,
/// }
/// ```
///
/// # Notes
/// - If a field type matches an existing struct name, the field type is set to the corresponding struct.
/// - If a field has no matching struct, its type is defaulted to `String`.
pub fn create_structs(xml_string: &str) -> String {
    let mut stack: Vec<XMLStruct> = Vec::new(); // Stack of structs being constructed
    let mut empty_structs: HashMap<String, XMLStruct> = HashMap::new(); // Structs from self-closing tags
    let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut field_counts: HashMap<String, HashMap<String, usize>> = HashMap::new(); // Count of fields per struct
    let mut max_counts: HashMap<String, HashMap<String, usize>> = HashMap::new(); // Maximum count of fields per struct
    let mut start_tags: Vec<String> = Vec::new(); // Start tags for elements

    let mut reader = Reader::from_str(xml_string);

    loop {
        match reader.read_event() {
            Ok(Start(ref e)) => {
                let element_name = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();

                start_tags.push(element_name.clone());

                // Create a new struct for this element
                let mut new_struct = XMLStruct {
                    name: element_name.clone(),
                    fields: Vec::new(),
                };
                
                parse_attributes(e.clone(), &mut new_struct);

                // If there's a parent struct, add this struct as a field to it
                if let Some(parent_struct) = stack.last_mut() {
                    let field_count = field_counts.entry(parent_struct.name.clone()).or_insert(HashMap::new());
                    let child_count = field_count.entry(element_name.clone()).or_insert(0);
                    *child_count += 1; // Because child_count and field_count are borrowed, field_counts is updated after this line

                    // Update max_counts for the current parent_struct
                    let parent_max_counts = max_counts.entry(parent_struct.name.clone()).or_insert_with(HashMap::new);

                    // Update the count for the child
                    let child_max_count = parent_max_counts.entry(element_name.clone()).or_insert(0);
                    if *child_count > *child_max_count {
                        *child_max_count = *child_count;
                    }

                    // Check that the parent doesn't contain a field with the same name
                    if !parent_struct.fields.iter().any(|field| field.name == element_name) {
                        parent_struct.fields.push(XMLField {
                            name: element_name.clone(),
                            field_type: element_name.clone(), // Use the same name as struct type
                        });
                    }
                }

                // Push this struct onto the stack if it's not already there
                if !stack.iter().any(|s| s.name == element_name) {
                    stack.push(new_struct);
                }
            },
            Ok(Empty(ref e)) => {
                let element_name = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();

                // Create a new struct for this element
                let mut new_struct = XMLStruct {
                    name: element_name.clone(),
                    fields: Vec::new(),
                };

                parse_attributes(e.clone(), &mut new_struct);

                empty_structs.insert(element_name.clone(), new_struct.clone());

                // If there's a parent struct, add this struct as a field to it
                if let Some(parent_struct) = stack.last_mut() {
                    if !parent_struct.fields.iter().any(|field| field.name == element_name) {
                        parent_struct.fields.push(XMLField {
                            name: element_name.clone(),
                            field_type: element_name.clone(),
                        });
                    }
                }
            },
            Ok(End(ref e)) => {
                let element_name = std::str::from_utf8(e.name().as_ref()).unwrap().to_string();

                // Pop the current struct from the stack
                if let Some(completed_struct) = stack.pop() {
                    if completed_struct.name != element_name {
                        panic!("XML structure mismatch: expected {}, found {}", completed_struct.name, element_name);
                    }

                    // Update the final struct with new fields or insert it if it doesn't exist
                    if let Some(existing_struct) = structs.get_mut(&completed_struct.name.clone()) {

                        // Merge fields: add only new unique fields
                        for field in completed_struct.fields {
                            if !existing_struct.fields.iter().any(|f| f.name == field.name) {
                                existing_struct.fields.push(field.clone());
                            }
                            
                            // Reset the field_count for this field
                            if let Some(field_count) = field_counts.get_mut(&field.name) {
                                field_count.clear();
                            } 
                        }
                    } else {

                        // No existing struct, insert the completed struct as it is
                        structs.insert(completed_struct.name.clone(), completed_struct.clone());
                    }

                    // Clear field counts for the parent element
                    field_counts.remove(&completed_struct.name);
                }
            },
            Ok(Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    remove_fieldless_structs(&mut structs);

    // Remove empty structs that have had content (start tags)
    empty_structs.retain(|name, _| !start_tags.contains(&name));

    add_empty_structs(&mut structs, &mut empty_structs);

    update_field_types(&mut structs, &max_counts);

    // Return the generated structs as a string
    generate_structs_string(&structs)
}

// Parse attributes and add them as fields
fn parse_attributes(
    e: quick_xml::events::BytesStart, 
    new_struct: &mut XMLStruct) {

    for attr in e.attributes() {
        if let Ok(attr) = attr {
            let attr_name = std::str::from_utf8(attr.key.as_ref()).unwrap().to_string();

            // Check if the attribute is a type attribute, if so, add _type to the field name
            let field_name = if attr_name == "type" {
                let element_name = e.name().0.to_vec().as_slice().to_vec().iter().map(|&c| c as char).collect::<String>();
                format!("@{}_type", to_snake_case(&to_camel_case_with_prefix(&element_name)))
            } else if attr_name == "xlink:type" {
                format!("@xlink_type")
            } else {
                format!("@{}", attr_name)
            };

            // Add attribute as a field to the current struct
            new_struct.fields.push(XMLField {
                name: field_name.clone(),
                field_type: "String".to_string(), // Attributes are strings
            });
        }
    }

    // Add text field
    new_struct.fields.push(XMLField {
        name: "$text".to_string(),
        field_type: "String".to_string(),
    });
}

// Removes the structs that don't have any fields
fn remove_fieldless_structs(structs: &mut HashMap<String, XMLStruct>) {
    let keys_to_remove: Vec<String> = structs
        .iter()
        .filter(|(_, xml_struct)| xml_struct.fields.len() == 1 && xml_struct.fields[0].name == "$text")
        .map(|(name, _)| name.clone())
        .collect();

    for key in keys_to_remove {
        structs.remove(&key);
    }
}

// Add the empty structs to the final structs
fn add_empty_structs(
    structs: &mut HashMap<String, XMLStruct>, 
    empty_structs: &mut HashMap<String, XMLStruct>
) {

    // Remove the $text field from empty structs
    for (_, xml_struct) in &mut *empty_structs {
        xml_struct.fields.retain(|field| field.name != "$text");
    }

    // Add the empty structs to the final structs
    for s in empty_structs.values() {
        if !structs.contains_key(&s.name) {
            structs.insert(s.name.clone(), s.clone());
        }
    }
}

// Update the field types for fields that occur more than once to Vec<T>
fn update_field_types(
    structs: &mut HashMap<String, XMLStruct>, 
    max_counts: &HashMap<String, HashMap<String, usize>>
) {

    for (parent_name, child_map) in max_counts {
        if let Some(parent_struct) = structs.get_mut(parent_name) {
            
            // Check for fields that occur more than once
            for (child_name, child_count) in child_map {
                if *child_count > 1 {

                    // Update the field type to Vec<T>
                    for field in &mut parent_struct.fields {
                        if field.name == *child_name {
                            field.field_type = format!("Vec<{}>", field.name);
                        }
                    }
                }
            }
        }
    } 
}

/// Creates structs from an XML string and saves them to a file
/// 
/// # Parameters
/// - `xml_string`: A string slice containing the XML document to parse.
/// - `file_name`: A string slice containing the name of the file to save the structs to.
pub fn create_structs_and_save_to_file(
    xml_string: &str, 
    file_name: &str
) {

    // Create string representation of structs from the XML string
    let struct_string = create_structs(xml_string);

    // Save the generated structs to a file
    let mut struct_file = File::create(file_name).unwrap();
    struct_file.write_all(&struct_string.as_bytes()).unwrap();

    println!("Structs saved to {}\n", file_name);
}