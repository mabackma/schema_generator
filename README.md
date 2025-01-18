## schema_generator

#### schema_generator is a Rust crate initially created for parsing XML files containing forestry data and generating corresponding Rust structs. It parses XML documents and automatically generates Rust data structures that map XML elements to fields, including attributes, child elements, and text content. It integrates with the serde library for serialization and deserialization. While it was originally designed for forestry-related XML data, it can be used with any XML data structure, making it a versatile tool for developers working with XML. Additionally, schema_generator supports JSON to XML conversion.

## Known bugs in Json to XML conversion:

#### If the first nested element has attributes and the parent element also has attributes, the nested element will sometimes inherit the same prefix from the parent (and sometimes update to its own prefix as its meant to). This is likely due to the recursive nature of the function.

## Reasons why the generated XML file might have fewer rows than the original XML file:

#### These are not exactly bugs, since the generated file will still contain all the relevant information from the original file.

- <ts:TreeStandDataDate> tags that don't have any content will be written on two rows instead of one.

- The tags and contents inside <gdt:PolygonGeometry> and <gdt:MultiPolygonGeometry> will be written on their own rows, even if they are on a single row in the original XML file.

## Other mentions

#### Some namespaces are not included in the XML. The following namespace prefixes were manually added:
- SpecialFeatures
- SpecialFeature
- TreeStrata

#### xlink:type attribute in <gml:polygonProperty> tags will show up as __type in Json

## License

[MIT License](LICENSE)