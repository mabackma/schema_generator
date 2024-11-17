#[derive(Serialize, Deserialize)]
pub struct Parcels {
	#[serde(rename = "Parcel", skip_serializing_if = "Option::is_none")]
	pub re_parcel: Option<Parcel>,
}

#[derive(Serialize, Deserialize)]
pub struct PolygonGeometry {
	#[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
	pub gml_point_property: Option<pointProperty>,
	#[serde(rename = "polygonProperty", skip_serializing_if = "Option::is_none")]
	pub gml_polygon_property: Option<polygonProperty>,
}

#[derive(Serialize, Deserialize)]
pub struct Assortment {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LinearRing {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub gml_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProposalData {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TreeStandDataDate {
	#[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
	pub tst_tree_strata: Option<TreeStrata>,
	#[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
	pub tss_tree_stand_summary: Option<TreeStandSummary>,
	#[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
	pub dts_dead_tree_strata: Option<DeadTreeStrata>,
}

#[derive(Serialize, Deserialize)]
pub struct SpecialFeature {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub sf_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub sf_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Specification {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Identifiers {
	#[serde(rename = "Identifier", skip_serializing_if = "Option::is_none")]
	pub st_identifier: Option<Identifier>,
}

#[derive(Serialize, Deserialize)]
pub struct Operations {
	#[serde(rename = "Operation", skip_serializing_if = "Option::is_none")]
	pub op_operation: Option<Operation>,
}

#[derive(Serialize, Deserialize)]
pub struct Stands {
	#[serde(rename = "Stand", skip_serializing_if = "Option::is_none")]
	pub st_stand: Option<Stand>,
}

#[derive(Serialize, Deserialize)]
pub struct Stand {
	#[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
	pub st_stand_basic_data: Option<StandBasicData>,
	#[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data: Option<TreeStandData>,
	#[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
	pub op_operations: Option<Operations>,
	#[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
	pub st_special_features: Option<SpecialFeatures>,
}

#[derive(Serialize, Deserialize)]
pub struct pointProperty {
	#[serde(rename = "Point", skip_serializing_if = "Option::is_none")]
	pub gml_point: Option<Point>,
}

#[derive(Serialize, Deserialize)]
pub struct SpecialFeatures {
	#[serde(rename = "SpecialFeature", skip_serializing_if = "Option::is_none")]
	pub st_special_feature: Option<SpecialFeature>,
}

#[derive(Serialize, Deserialize)]
pub struct Polygon {
	#[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
	pub gml_exterior: Option<exterior>,
	#[serde(rename = "interior", skip_serializing_if = "Option::is_none")]
	pub gml_interior: Option<interior>,
}

#[derive(Serialize, Deserialize)]
pub struct Identifier {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RealEstate {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub re_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub re_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub re_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub re_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub re_string: Option<String>,
	#[serde(rename = "Parcels", skip_serializing_if = "Option::is_none")]
	pub re_parcels: Option<Parcels>,
}

#[derive(Serialize, Deserialize)]
pub struct Assortments {
	#[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
	pub op_assortment: Option<Assortment>,
}

#[derive(Serialize, Deserialize)]
pub struct Parcel {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub re_string: Option<String>,
	#[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
	pub st_stands: Option<Stands>,
}

#[derive(Serialize, Deserialize)]
pub struct RealEstates {
	#[serde(rename = "RealEstate", skip_serializing_if = "Option::is_none")]
	pub re_real_estate: Option<RealEstate>,
}

#[derive(Serialize, Deserialize)]
pub struct Point {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub gml_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct exterior {
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<LinearRing>,
}

#[derive(Serialize, Deserialize)]
pub struct DeadTreeStratum {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub dts_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub dts_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub dts_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub dts_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ForestPropertyData {
	#[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
	pub re_real_estates: Option<RealEstates>,
}

#[derive(Serialize, Deserialize)]
pub struct TreeStrata {
	#[serde(rename = "TreeStratum", skip_serializing_if = "Option::is_none")]
	pub tst_tree_stratum: Option<TreeStratum>,
}

#[derive(Serialize, Deserialize)]
pub struct CompletionData {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TreeStandData {
	#[serde(rename = "TreeStandDataDate", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data_date: Option<TreeStandDataDate>,
}

#[derive(Serialize, Deserialize)]
pub struct polygonProperty {
	#[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
	pub gml_polygon: Option<Polygon>,
}

#[derive(Serialize, Deserialize)]
pub struct StandBasicData {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "PolygonGeometry", skip_serializing_if = "Option::is_none")]
	pub gdt_polygon_geometry: Option<PolygonGeometry>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
	#[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
	pub st_identifiers: Option<Identifiers>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub st_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TreeStratum {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tst_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct interior {
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<LinearRing>,
}

#[derive(Serialize, Deserialize)]
pub struct Specifications {
	#[serde(rename = "Specification", skip_serializing_if = "Option::is_none")]
	pub op_specification: Option<Specification>,
}

#[derive(Serialize, Deserialize)]
pub struct TreeStandSummary {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub tss_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Operation {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub co_string: Option<String>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
	#[serde(rename = "ProposalData", skip_serializing_if = "Option::is_none")]
	pub op_proposal_data: Option<ProposalData>,
	#[serde(rename = "Cutting", skip_serializing_if = "Option::is_none")]
	pub op_cutting: Option<Cutting>,
	#[serde(rename = "Specifications", skip_serializing_if = "Option::is_none")]
	pub op_specifications: Option<Specifications>,
	#[serde(rename = "CompletionData", skip_serializing_if = "Option::is_none")]
	pub op_completion_data: Option<CompletionData>,
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeadTreeStrata {
	#[serde(rename = "DeadTreeStratum", skip_serializing_if = "Option::is_none")]
	pub dts_dead_tree_stratum: Option<DeadTreeStratum>,
}

#[derive(Serialize, Deserialize)]
pub struct Cutting {
	#[serde(rename = "String", skip_serializing_if = "Option::is_none")]
	pub op_string: Option<String>,
	#[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
	pub op_assortments: Option<Assortments>,
}

