use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPropertyData {
	#[serde(rename = "@xmlns")]
	pub xmlns: String,
	#[serde(rename = "@xmlns:re")]
	pub xmlns_re: String,
	#[serde(rename = "@xmlns:st")]
	pub xmlns_st: String,
	#[serde(rename = "@xmlns:ts")]
	pub xmlns_ts: String,
	#[serde(rename = "@xmlns:tst")]
	pub xmlns_tst: String,
	#[serde(rename = "@xmlns:dts")]
	pub xmlns_dts: String,
	#[serde(rename = "@xmlns:tss")]
	pub xmlns_tss: String,
	#[serde(rename = "@xmlns:op")]
	pub xmlns_op: String,
	#[serde(rename = "@xmlns:sf")]
	pub xmlns_sf: String,
	#[serde(rename = "@xmlns:gdt")]
	pub xmlns_gdt: String,
	#[serde(rename = "@xmlns:co")]
	pub xmlns_co: String,
	#[serde(rename = "@xmlns:gml")]
	pub xmlns_gml: String,
	#[serde(rename = "@xmlns:xsi")]
	pub xmlns_xsi: String,
	#[serde(rename = "@xmlns:xlink")]
	pub xmlns_xlink: String,
	#[serde(rename = "@schemaLocation")]
	pub xsi_schema_location: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
	pub re_real_estates: Option<ReRealEstates>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlpolygonProperty {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
	pub gml_polygon: Option<GmlPolygon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpSpecifications {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Specification", skip_serializing_if = "Option::is_none")]
	pub op_specification: Option<Vec<OpSpecification>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpCompletionData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "CompletionDate", skip_serializing_if = "Option::is_none")]
	pub op_completion_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TstTreeStratum {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "StratumNumber", skip_serializing_if = "Option::is_none")]
	pub tst_stratum_number: Option<String>,
	#[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
	pub tst_tree_species: Option<String>,
	#[serde(rename = "Storey", skip_serializing_if = "Option::is_none")]
	pub tst_storey: Option<String>,
	#[serde(rename = "Age", skip_serializing_if = "Option::is_none")]
	pub tst_age: Option<String>,
	#[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
	pub tst_basal_area: Option<String>,
	#[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
	pub tst_mean_diameter: Option<String>,
	#[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
	pub tst_mean_height: Option<String>,
	#[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
	pub co_data_source: Option<String>,
	#[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
	pub tst_stem_count: Option<String>,
	#[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
	pub tst_volume: Option<String>,
	#[serde(rename = "SawLogPercent", skip_serializing_if = "Option::is_none")]
	pub tst_saw_log_percent: Option<String>,
	#[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
	pub tst_saw_log_volume: Option<String>,
	#[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
	pub tst_pulp_wood_volume: Option<String>,
	#[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
	pub tst_volume_growth: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TsTreeStandDataDate {
	#[serde(rename = "@date")]
	pub date: String,
	#[serde(rename = "@type")]
	pub ts_tree_stand_data_date_type: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
	pub tst_tree_strata: Option<TstTreeStrata>,
	#[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
	pub tss_tree_stand_summary: Option<TssTreeStandSummary>,
	#[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
	pub dts_dead_tree_strata: Option<DtsDeadTreeStrata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StSpecialFeature {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "FeatureCode", skip_serializing_if = "Option::is_none")]
	pub sf_feature_code: Option<String>,
	#[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
	pub sf_feature_additional_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpAssortment {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
	pub op_tree_species: Option<String>,
	#[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
	pub op_stem_type: Option<String>,
	#[serde(rename = "AssortmentVolume", skip_serializing_if = "Option::is_none")]
	pub op_assortment_volume: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpSpecification {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "SpecificationCode", skip_serializing_if = "Option::is_none")]
	pub op_specification_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TstTreeStrata {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "TreeStratum", skip_serializing_if = "Option::is_none")]
	pub tst_tree_stratum: Option<Vec<TstTreeStratum>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TsTreeStandData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "TreeStandDataDate", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data_date: Option<Vec<TsTreeStandDataDate>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpProposalData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ProposalType", skip_serializing_if = "Option::is_none")]
	pub op_proposal_type: Option<String>,
	#[serde(rename = "ProposalYear", skip_serializing_if = "Option::is_none")]
	pub op_proposal_year: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlPolygon {
	#[serde(rename = "@srsName")]
	pub srs_name: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
	pub gml_exterior: Option<Gmlexterior>,
	#[serde(rename = "interior", skip_serializing_if = "Option::is_none")]
	pub gml_interior: Option<Vec<Gmlinterior>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GdtPolygonGeometry {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
	pub gml_point_property: Option<GmlpointProperty>,
	#[serde(rename = "polygonProperty", skip_serializing_if = "Option::is_none")]
	pub gml_polygon_property: Option<GmlpolygonProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StSpecialFeatures {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "SpecialFeature", skip_serializing_if = "Option::is_none")]
	pub st_special_feature: Option<Vec<StSpecialFeature>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpOperation {
	#[serde(rename = "@mainType")]
	pub main_type: String,
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
	pub co_change_time: Option<String>,
	#[serde(rename = "OperationType", skip_serializing_if = "Option::is_none")]
	pub op_operation_type: Option<String>,
	#[serde(rename = "ProposalData", skip_serializing_if = "Option::is_none")]
	pub op_proposal_data: Option<OpProposalData>,
	#[serde(rename = "Cutting", skip_serializing_if = "Option::is_none")]
	pub op_cutting: Option<OpCutting>,
	#[serde(rename = "Silviculture", skip_serializing_if = "Option::is_none")]
	pub op_silviculture: Option<OpSilviculture>,
	#[serde(rename = "Specifications", skip_serializing_if = "Option::is_none")]
	pub op_specifications: Option<OpSpecifications>,
	#[serde(rename = "CompletionData", skip_serializing_if = "Option::is_none")]
	pub op_completion_data: Option<OpCompletionData>,
	#[serde(rename = "OperationInfo", skip_serializing_if = "Option::is_none")]
	pub op_operation_info: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReRealEstates {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "RealEstate", skip_serializing_if = "Option::is_none")]
	pub re_real_estate: Option<ReRealEstate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StIdentifier {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "IdentifierType", skip_serializing_if = "Option::is_none")]
	pub co_identifier_type: Option<String>,
	#[serde(rename = "IdentifierValue", skip_serializing_if = "Option::is_none")]
	pub co_identifier_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtsDeadTreeStratum {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "DeadTreeType", skip_serializing_if = "Option::is_none")]
	pub dts_dead_tree_type: Option<String>,
	#[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
	pub dts_tree_species: Option<String>,
	#[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
	pub dts_mean_diameter: Option<String>,
	#[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
	pub dts_volume: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtsDeadTreeStrata {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "DeadTreeStratum", skip_serializing_if = "Option::is_none")]
	pub dts_dead_tree_stratum: Option<Vec<DtsDeadTreeStratum>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gmlexterior {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<GmlLinearRing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpOperations {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Operation", skip_serializing_if = "Option::is_none")]
	pub op_operation: Option<Vec<OpOperation>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StIdentifiers {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Identifier", skip_serializing_if = "Option::is_none")]
	pub st_identifier: Option<StIdentifier>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StStands {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Stand", skip_serializing_if = "Option::is_none")]
	pub st_stand: Option<Vec<StStand>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReParcels {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Parcel", skip_serializing_if = "Option::is_none")]
	pub re_parcel: Option<Vec<ReParcel>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlPoint {
	#[serde(rename = "@srsName")]
	pub srs_name: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
	pub gml_coordinates: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpAssortments {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
	pub op_assortment: Option<Vec<OpAssortment>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StStandBasicData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
	pub co_change_time: Option<String>,
	#[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
	pub st_complete_state: Option<String>,
	#[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
	pub st_stand_number: Option<String>,
	#[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
	pub st_stand_number_extension: Option<String>,
	#[serde(rename = "MainGroup", skip_serializing_if = "Option::is_none")]
	pub st_main_group: Option<String>,
	#[serde(rename = "SubGroup", skip_serializing_if = "Option::is_none")]
	pub st_sub_group: Option<String>,
	#[serde(rename = "FertilityClass", skip_serializing_if = "Option::is_none")]
	pub st_fertility_class: Option<String>,
	#[serde(rename = "SoilType", skip_serializing_if = "Option::is_none")]
	pub st_soil_type: Option<String>,
	#[serde(rename = "DrainageState", skip_serializing_if = "Option::is_none")]
	pub st_drainage_state: Option<String>,
	#[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
	pub st_development_class: Option<String>,
	#[serde(rename = "StandQuality", skip_serializing_if = "Option::is_none")]
	pub st_stand_quality: Option<String>,
	#[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
	pub st_main_tree_species: Option<String>,
	#[serde(rename = "Accessibility", skip_serializing_if = "Option::is_none")]
	pub st_accessibility: Option<String>,
	#[serde(rename = "StandBasicDataDate", skip_serializing_if = "Option::is_none")]
	pub st_stand_basic_data_date: Option<String>,
	#[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
	pub st_area: Option<String>,
	#[serde(rename = "PolygonGeometry", skip_serializing_if = "Option::is_none")]
	pub gdt_polygon_geometry: Option<GdtPolygonGeometry>,
	#[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
	pub st_stand_info: Option<String>,
	#[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
	pub st_area_decrease: Option<String>,
	#[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
	pub st_ditching_year: Option<String>,
	#[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
	pub st_identifiers: Option<StIdentifiers>,
	#[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
	pub st_cutting_restriction: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpCutting {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "CuttingVolume", skip_serializing_if = "Option::is_none")]
	pub op_cutting_volume: Option<String>,
	#[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
	pub op_assortments: Option<OpAssortments>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReParcel {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ParcelNumber", skip_serializing_if = "Option::is_none")]
	pub re_parcel_number: Option<String>,
	#[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
	pub st_stands: Option<StStands>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReRealEstate {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "MunicipalityNumber", skip_serializing_if = "Option::is_none")]
	pub re_municipality_number: Option<String>,
	#[serde(rename = "AreaNumber", skip_serializing_if = "Option::is_none")]
	pub re_area_number: Option<String>,
	#[serde(rename = "GroupNumber", skip_serializing_if = "Option::is_none")]
	pub re_group_number: Option<String>,
	#[serde(rename = "UnitNumber", skip_serializing_if = "Option::is_none")]
	pub re_unit_number: Option<String>,
	#[serde(rename = "RealEstateName", skip_serializing_if = "Option::is_none")]
	pub re_real_estate_name: Option<String>,
	#[serde(rename = "Parcels", skip_serializing_if = "Option::is_none")]
	pub re_parcels: Option<ReParcels>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TssTreeStandSummary {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "MeanAge", skip_serializing_if = "Option::is_none")]
	pub tss_mean_age: Option<String>,
	#[serde(rename = "BasalArea", skip_serializing_if = "Option::is_none")]
	pub tss_basal_area: Option<String>,
	#[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
	pub tss_stem_count: Option<String>,
	#[serde(rename = "MeanDiameter", skip_serializing_if = "Option::is_none")]
	pub tss_mean_diameter: Option<String>,
	#[serde(rename = "MeanHeight", skip_serializing_if = "Option::is_none")]
	pub tss_mean_height: Option<String>,
	#[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
	pub tss_volume: Option<String>,
	#[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
	pub tss_volume_growth: Option<String>,
	#[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
	pub tss_value: Option<String>,
	#[serde(rename = "ValueGrowthPercent", skip_serializing_if = "Option::is_none")]
	pub tss_value_growth_percent: Option<String>,
	#[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
	pub tss_saw_log_volume: Option<String>,
	#[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
	pub tss_pulp_wood_volume: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlpointProperty {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Point", skip_serializing_if = "Option::is_none")]
	pub gml_point: Option<GmlPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gmlinterior {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<GmlLinearRing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlLinearRing {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
	pub gml_coordinates: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpSilviculture {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StStand {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
	pub st_stand_basic_data: Option<StStandBasicData>,
	#[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data: Option<TsTreeStandData>,
	#[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
	pub op_operations: Option<OpOperations>,
	#[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
	pub st_special_features: Option<StSpecialFeatures>,
}

