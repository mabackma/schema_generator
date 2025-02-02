// Generated with schema_generator 0.0.0
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StStands {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Stand", skip_serializing_if = "Option::is_none")]
	pub st_stand: Option<Vec<StStand>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TstTreeStratum {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
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
	#[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
	pub tst_volume: Option<String>,
	#[serde(rename = "StemCount", skip_serializing_if = "Option::is_none")]
	pub tst_stem_count: Option<String>,
	#[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
	pub tst_saw_log_volume: Option<String>,
	#[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
	pub tst_pulp_wood_volume: Option<String>,
	#[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
	pub tst_volume_growth: Option<String>,
	#[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
	pub tst_leaf_biomass: Option<String>,
	#[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
	pub tst_branch_biomass: Option<String>,
	#[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
	pub tst_stem_biomass: Option<String>,
	#[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
	pub tst_stump_biomass: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlpolygonProperty {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
	pub gml_polygon: Option<GmlPolygon>,
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
pub struct GmlLinearRing {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
	pub gml_coordinates: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlpointProperty {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Point", skip_serializing_if = "Option::is_none")]
	pub gml_point: Option<GmlPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gmlexterior {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<GmlLinearRing>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TssTreeStandSummary {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
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
	#[serde(rename = "SawLogVolume", skip_serializing_if = "Option::is_none")]
	pub tss_saw_log_volume: Option<String>,
	#[serde(rename = "PulpWoodVolume", skip_serializing_if = "Option::is_none")]
	pub tss_pulp_wood_volume: Option<String>,
	#[serde(rename = "VolumeGrowth", skip_serializing_if = "Option::is_none")]
	pub tss_volume_growth: Option<String>,
	#[serde(rename = "LeafBiomass", skip_serializing_if = "Option::is_none")]
	pub tss_leaf_biomass: Option<String>,
	#[serde(rename = "BranchBiomass", skip_serializing_if = "Option::is_none")]
	pub tss_branch_biomass: Option<String>,
	#[serde(rename = "StemBiomass", skip_serializing_if = "Option::is_none")]
	pub tss_stem_biomass: Option<String>,
	#[serde(rename = "StumpBiomass", skip_serializing_if = "Option::is_none")]
	pub tss_stump_biomass: Option<String>,
	#[serde(rename = "DevelopmentClass", skip_serializing_if = "Option::is_none")]
	pub tss_development_class: Option<String>,
	#[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
	pub tss_main_tree_species: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GmlPolygon {
	#[serde(rename = "@srsName", skip_serializing_if = "Option::is_none")]
	pub srs_name: Option<String>,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
	pub gml_exterior: Option<Gmlexterior>,
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
pub struct TstTreeStrata {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "TreeStratum", skip_serializing_if = "Option::is_none")]
	pub tst_tree_stratum: Option<Vec<TstTreeStratum>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StIdentifiers {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "Identifier", skip_serializing_if = "Option::is_none")]
	pub st_identifier: Option<Vec<StIdentifier>>,
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
pub struct StStand {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
	pub st_stand_basic_data: Option<StStandBasicData>,
	#[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data: Option<TsTreeStandData>,
	#[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
	pub st_special_features: Option<StSpecialFeatures>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StStandBasicData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
	pub st_complete_state: Option<String>,
	#[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
	pub st_identifiers: Option<StIdentifiers>,
	#[serde(rename = "StandNumber", skip_serializing_if = "Option::is_none")]
	pub st_stand_number: Option<String>,
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
	#[serde(rename = "MainTreeSpecies", skip_serializing_if = "Option::is_none")]
	pub st_main_tree_species: Option<String>,
	#[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
	pub st_cutting_restriction: Option<String>,
	#[serde(rename = "SilvicultureRestriction", skip_serializing_if = "Option::is_none")]
	pub st_silviculture_restriction: Option<String>,
	#[serde(rename = "StandBasicDataDate", skip_serializing_if = "Option::is_none")]
	pub st_stand_basic_data_date: Option<String>,
	#[serde(rename = "DataSource", skip_serializing_if = "Option::is_none")]
	pub co_data_source: Option<String>,
	#[serde(rename = "GrowthPlaceDataSource", skip_serializing_if = "Option::is_none")]
	pub st_growth_place_data_source: Option<String>,
	#[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
	pub st_area: Option<String>,
	#[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
	pub st_area_decrease: Option<String>,
	#[serde(rename = "PolygonGeometry", skip_serializing_if = "Option::is_none")]
	pub gdt_polygon_geometry: Option<GdtPolygonGeometry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StSpecialFeatures {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "SpecialFeature", skip_serializing_if = "Option::is_none")]
	pub st_special_feature: Option<StSpecialFeature>,
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
pub struct StSpecialFeature {
	#[serde(rename = "@id")]
	pub id: String,
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "MainFeature", skip_serializing_if = "Option::is_none")]
	pub sf_main_feature: Option<String>,
	#[serde(rename = "FeatureCode", skip_serializing_if = "Option::is_none")]
	pub sf_feature_code: Option<String>,
	#[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
	pub sf_feature_additional_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TsTreeStandData {
	#[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(rename = "TreeStandDataDate", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data_date: Option<Vec<TsTreeStandDataDate>>,
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
}

