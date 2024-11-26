use crate::string_utils::{to_camel_case_with_prefix, to_snake_case};

use std::collections::HashMap;
use quick_xml::events::Event::{Start, Empty, End, Eof};
use quick_xml::reader::Reader;

#[derive(Debug, Clone)]
pub struct XMLField {
    pub name: String,
    pub field_type: String,
}

#[derive(Debug)]
pub struct XMLStruct {
    name: String,
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

// Create structs from the XML document
pub fn create_structs(reader: &mut Reader<&[u8]>) -> HashMap<String, XMLStruct> {
    let mut stack: Vec<XMLStruct> = Vec::new(); // Stack of structs being constructed
    let mut empty_structs: HashMap<String, XMLStruct> = HashMap::new(); // Structs from self-closing tags
    let mut structs: HashMap<String, XMLStruct> = HashMap::new(); // Finalized structs
    let mut field_counts: HashMap<String, HashMap<String, usize>> = HashMap::new(); // Count of fields per struct
    let mut max_counts: HashMap<String, HashMap<String, usize>> = HashMap::new(); // Maximum count of fields per struct
    let mut start_tags: Vec<String> = Vec::new(); // Start tags for elements

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

    structs
}

// Parse attributes and add them as fields
fn parse_attributes(e: quick_xml::events::BytesStart, new_struct: &mut XMLStruct) {
    for attr in e.attributes() {
        if let Ok(attr) = attr {
            let attr_name = std::str::from_utf8(attr.key.as_ref()).unwrap().to_string();

            // Check if the attribute is a type attribute, if so, add _type to the field name
            let field_name = if attr_name == "type" {
                let element_name = e.name().0.to_vec().as_slice().to_vec().iter().map(|&c| c as char).collect::<String>();
                format!("@{}_type", to_snake_case(&to_camel_case_with_prefix(&element_name)))
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
fn add_empty_structs(structs: &mut HashMap<String, XMLStruct>, empty_structs: &mut HashMap<String, XMLStruct>) {
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
fn update_field_types(structs: &mut HashMap<String, XMLStruct>, max_counts: &HashMap<String, HashMap<String, usize>>) {
    for (parent_name, child_map) in max_counts {
        if let Some(parent_struct) = structs.get_mut(parent_name) {
            // Check for fields that occur more than once
            for (child_name, child_count) in child_map {
                if *child_count > 1 {
                    println!("--------{}: {} -> {}", parent_name, child_name, child_count);

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