## schema_generator
- Converts XML file into data structures in Rust (structs) by reading the file forestpropertydata.xml located in the root of the project. The structs are saved in the file src/file_structs.rs.
- Converts XML that is fetched from API into structs and saves the the structs to the file src/url_structs.rs.
- Uses the structs to read the XML file's data and creates a Json file containing that data using quick-xml and Serde.
- Converts the Json file back into XML using a recursive function. The recursive function is not dependent on the structs.

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