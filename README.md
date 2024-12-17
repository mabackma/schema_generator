## Known bugs in Json to XML conversion:

#### If the first nested element has attributes and the parent element also has attributes, the nested element will sometimes inherit the same prefix from the parent (and sometimes update to its own prefix as its meant to). This is likely due to the recursive nature of the function.

## Other mentions

#### Some namespaces are not included in the XML. The following namespace prefixes were manually added:
- SpecialFeatures
- SpecialFeature
- TreeStrata
