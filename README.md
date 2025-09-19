## schema_generator

#### `schema_generator` is a Rust crate initially created for parsing XML files containing forestry data and generating corresponding Rust structs. It parses XML documents and automatically generates Rust data structures that map XML elements to fields, including attributes, child elements, and text content. It integrates with the serde library for serialization and deserialization. While it was originally designed for forestry-related XML data, it can be used with any XML data structure, making it a versatile tool for developers working with XML. Additionally, schema_generator supports JSON to XML conversion.

[Crate](https://crates.io/crates/schema_generator)

## Features
- `create_structs_and_save_to_file`: creates structs from an XML String and saves them to a file into the /src folder, making the structs ready for use in an application. When use_primitives is set to true, numeric fields will have type Number (e.g. i64 or f64). When it is set to false, all fields will have String type.
- `json_keys_to_lowercase`: converts all keys in a serde_json value to lowercase. Additionally, it replaces all @ characters with __ to avoid unexpected behavior during JSON processing.
- `json_to_xml`: converts JSON data into XML format. Attribute keys in the JSON are expected to be marked with __.

## Usage

#### You can find integration tests and example usages of this crate in the [Integration Test Repository](https://github.com/mabackma/forestry_structs).

## Known bugs in JSON to XML conversion:

#### If the first nested element has attributes and the parent element also has attributes, the nested element will sometimes inherit the same prefix from the parent (and sometimes update to its own prefix as it's meant to). This is likely due to the recursive nature of the function.

## Other mentions

#### Some namespaces are not included in the XML. The following namespace prefixes were manually added:
- SpecialFeatures
- SpecialFeature
- TreeStrata

#### The field for IdentifierValue will be given type `Option<String>` even when `use_primitives` is set to true.

#### xlink:type attribute in <gml:polygonProperty> tags will show up as __type in Json

## License

[MIT License](LICENSE)
