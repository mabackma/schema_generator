## What this program does
- Converts XML file into data structures in Rust (structs) by reading the file forestpropertydata.xml located in the root of the project. The structs are saved in a file called file_structs.rs.
- Converts XML that is fetched from API into structs and saves the the structs to a filed called url_structs.rs.
- Uses the structs to read the XML file's data and create a Json file containing that data using serde.
- Converts the Json file back into XML using a recursive function. The recursive function is not dependant on the structs.

## Known bugs in Json to XML conversion:

#### If the first nested element has attributes and the parent element also has attributes, the nested element will sometimes inherit the same prefix from the parent (and sometimes update to its own prefix as its meant to). This is likely due to the recursive nature of the function.

## Other mentions

#### Some namespaces are not included in the XML. The following namespace prefixes were manually added:
- SpecialFeatures
- SpecialFeature
- TreeStrata
