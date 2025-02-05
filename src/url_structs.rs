// Generated with schema_generator 0.0.0
use crate::number::{Number, deserialize_optional_number};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpCompletionData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "CompletionDate", skip_serializing_if = "Option::is_none")]
	pub op_completion_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StIdentifiers {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Identifier", skip_serializing_if = "Option::is_none")]
	pub st_identifier: Option<Vec<StIdentifier>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpAssortments {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
	pub op_assortment: Option<Vec<OpAssortment>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TstTreeStratum {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "StratumNumber", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_stratum_number: Option<Number>,
	#[serde(rename = "TreeSpecies", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_tree_species: Option<Number>,
	#[serde(rename = "Storey", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_storey: Option<Number>,
	#[serde(rename = "Age", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_age: Option<Number>,
	#[serde(rename = "BasalArea", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_basal_area: Option<Number>,
	#[serde(rename = "MeanDiameter", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_mean_diameter: Option<Number>,
	#[serde(rename = "MeanHeight", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_mean_height: Option<Number>,
	#[serde(rename = "Volume", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_volume: Option<Number>,
	#[serde(rename = "StemCount", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_stem_count: Option<Number>,
	#[serde(rename = "SawLogVolume", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_saw_log_volume: Option<Number>,
	#[serde(rename = "PulpWoodVolume", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_pulp_wood_volume: Option<Number>,
	#[serde(rename = "VolumeGrowth", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_volume_growth: Option<Number>,
	#[serde(rename = "LeafBiomass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_leaf_biomass: Option<Number>,
	#[serde(rename = "BranchBiomass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_branch_biomass: Option<Number>,
	#[serde(rename = "StemBiomass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_stem_biomass: Option<Number>,
	#[serde(rename = "StumpBiomass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tst_stump_biomass: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtsDeadTreeStratum {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "DeadTreeType", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub dts_dead_tree_type: Option<Number>,
	#[serde(rename = "TreeSpecies", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub dts_tree_species: Option<Number>,
	#[serde(rename = "MeanDiameter", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub dts_mean_diameter: Option<Number>,
	#[serde(rename = "Volume", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub dts_volume: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TstTreeStrata {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "TreeStratum", skip_serializing_if = "Option::is_none")]
	pub tst_tree_stratum: Option<Vec<TstTreeStratum>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpAssortment {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "TreeSpecies", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub op_tree_species: Option<Number>,
	#[serde(rename = "StemType", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub op_stem_type: Option<Number>,
	#[serde(rename = "AssortmentPercent", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub op_assortment_percent: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gmlexterior {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<GmlLinearRing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DtsDeadTreeStrata {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "DeadTreeStratum", skip_serializing_if = "Option::is_none")]
	pub dts_dead_tree_stratum: Option<Vec<DtsDeadTreeStratum>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpOperations {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Operation", skip_serializing_if = "Option::is_none")]
	pub op_operation: Option<Vec<OpOperation>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpCutting {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "CuttingVolume", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub op_cutting_volume: Option<Number>,
	#[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
	pub op_assortments: Option<OpAssortments>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TssTreeStandSummary {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "MeanAge", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_mean_age: Option<Number>,
	#[serde(rename = "BasalArea", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_basal_area: Option<Number>,
	#[serde(rename = "StemCount", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_stem_count: Option<Number>,
	#[serde(rename = "MeanDiameter", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_mean_diameter: Option<Number>,
	#[serde(rename = "MeanHeight", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_mean_height: Option<Number>,
	#[serde(rename = "Volume", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_volume: Option<Number>,
	#[serde(rename = "SawLogVolume", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_saw_log_volume: Option<Number>,
	#[serde(rename = "PulpWoodVolume", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_pulp_wood_volume: Option<Number>,
	#[serde(rename = "VolumeGrowth", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_volume_growth: Option<Number>,
	#[serde(rename = "LeafBiomass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_leaf_biomass: Option<Number>,
	#[serde(rename = "BranchBiomass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_branch_biomass: Option<Number>,
	#[serde(rename = "StemBiomass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_stem_biomass: Option<Number>,
	#[serde(rename = "StumpBiomass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_stump_biomass: Option<Number>,
	#[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
	pub tss_development_class: Option<String>,
	#[serde(rename = "MainTreeSpecies", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub tss_main_tree_species: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StIdentifier {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "IdentifierType", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub co_identifier_type: Option<Number>,
	#[serde(rename = "IdentifierValue", skip_serializing_if = "Option::is_none")]
	pub co_identifier_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlLinearRing {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
	pub gml_coordinates: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StStandBasicData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "CompleteState", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_complete_state: Option<Number>,
	#[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
	pub st_identifiers: Option<StIdentifiers>,
	#[serde(rename = "StandNumber", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_stand_number: Option<Number>,
	#[serde(rename = "MainGroup", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_main_group: Option<Number>,
	#[serde(rename = "SubGroup", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_sub_group: Option<Number>,
	#[serde(rename = "FertilityClass", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_fertility_class: Option<Number>,
	#[serde(rename = "SoilType", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_soil_type: Option<Number>,
	#[serde(rename = "DrainageState", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_drainage_state: Option<Number>,
	#[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
	pub st_development_class: Option<String>,
	#[serde(rename = "MainTreeSpecies", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_main_tree_species: Option<Number>,
	#[serde(rename = "Accessibility", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_accessibility: Option<Number>,
	#[serde(rename = "StandBasicDataDate", skip_serializing_if = "Option::is_none")]
	pub st_stand_basic_data_date: Option<String>,
	#[serde(rename = "DataSource", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub co_data_source: Option<Number>,
	#[serde(rename = "GrowthPlaceDataSource", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_growth_place_data_source: Option<Number>,
	#[serde(rename = "Area", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_area: Option<Number>,
	#[serde(rename = "AreaDecrease", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_area_decrease: Option<Number>,
	#[serde(rename = "PolygonGeometry", skip_serializing_if = "Option::is_none")]
	pub gdt_polygon_geometry: Option<GdtPolygonGeometry>,
	#[serde(rename = "CuttingRestriction", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_cutting_restriction: Option<Number>,
	#[serde(rename = "SilvicultureRestriction", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_silviculture_restriction: Option<Number>,
	#[serde(rename = "DitchingYear", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_ditching_year: Option<Number>,
	#[serde(rename = "StandNumberExtension", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub st_stand_number_extension: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpOperation {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "@mainType")]
	pub main_type: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "OperationType", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub op_operation_type: Option<Number>,
	#[serde(rename = "ProposalData", skip_serializing_if = "Option::is_none")]
	pub op_proposal_data: Option<OpProposalData>,
	#[serde(rename = "Cutting", skip_serializing_if = "Option::is_none")]
	pub op_cutting: Option<OpCutting>,
	#[serde(rename = "CompletionData", skip_serializing_if = "Option::is_none")]
	pub op_completion_data: Option<OpCompletionData>,
	#[serde(rename = "DataSource", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub co_data_source: Option<Number>,
	#[serde(rename = "Silviculture", skip_serializing_if = "Option::is_none")]
	pub op_silviculture: Option<OpSilviculture>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TsTreeStandDataDate {
	#[serde(rename = "@type")]
	pub ts_tree_stand_data_date_type: String,
	#[serde(rename = "@date")]
	pub date: String,
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
pub struct GmlPolygon {
	#[serde(rename = "@srsName", skip_serializing_if = "Option::is_none")]
	pub srs_name: Option<String>,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
	pub gml_exterior: Option<Gmlexterior>,
	#[serde(rename = "interior", skip_serializing_if = "Option::is_none")]
	pub gml_interior: Option<Gmlinterior>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlpointProperty {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Point", skip_serializing_if = "Option::is_none")]
	pub gml_point: Option<GmlPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StSpecialFeature {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "MainFeature", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub sf_main_feature: Option<Number>,
	#[serde(rename = "FeatureCode", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub sf_feature_code: Option<Number>,
	#[serde(rename = "FeatureAdditionalCode", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub sf_feature_additional_code: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TsTreeStandData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "TreeStandDataDate", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data_date: Option<Vec<TsTreeStandDataDate>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gmlinterior {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<GmlLinearRing>,
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
pub struct OpSilviculture {
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StSpecialFeatures {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "SpecialFeature", skip_serializing_if = "Option::is_none")]
	pub st_special_feature: Option<Vec<StSpecialFeature>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlpolygonProperty {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
	pub gml_polygon: Option<GmlPolygon>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlPoint {
	#[serde(rename = "@srsName", skip_serializing_if = "Option::is_none")]
	pub srs_name: Option<String>,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
	pub gml_coordinates: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpProposalData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "ProposalType", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub op_proposal_type: Option<Number>,
	#[serde(rename = "ProposalYear", deserialize_with = "deserialize_optional_number", skip_serializing_if = "Option::is_none", default)]
	pub op_proposal_year: Option<Number>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForestPropertyData {
	#[serde(rename = "@schemaLocation")]
	pub xsi_schema_location: String,
	#[serde(rename = "@xmlns:xsi")]
	pub xmlns_xsi: String,
	#[serde(rename = "@xmlns:xlink")]
	pub xmlns_xlink: String,
	#[serde(rename = "@xmlns:gml")]
	pub xmlns_gml: String,
	#[serde(rename = "@xmlns:gdt")]
	pub xmlns_gdt: String,
	#[serde(rename = "@xmlns:co")]
	pub xmlns_co: String,
	#[serde(rename = "@xmlns:sf")]
	pub xmlns_sf: String,
	#[serde(rename = "@xmlns:op")]
	pub xmlns_op: String,
	#[serde(rename = "@xmlns:dts")]
	pub xmlns_dts: String,
	#[serde(rename = "@xmlns:tss")]
	pub xmlns_tss: String,
	#[serde(rename = "@xmlns:tst")]
	pub xmlns_tst: String,
	#[serde(rename = "@xmlns:ts")]
	pub xmlns_ts: String,
	#[serde(rename = "@xmlns:st")]
	pub xmlns_st: String,
	#[serde(rename = "@schemaPackageVersion")]
	pub schema_package_version: String,
	#[serde(rename = "@schemaPackageSubversion")]
	pub schema_package_subversion: String,
	#[serde(rename = "@xmlns")]
	pub xmlns: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
	pub st_stands: Option<StStands>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StStands {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Stand", skip_serializing_if = "Option::is_none")]
	pub st_stand: Option<Vec<StStand>>,
}

