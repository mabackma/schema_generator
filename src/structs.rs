#[derive(Serialize, Deserialize)]
pub struct GmlLinearRing {
	#[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
	pub gml_coordinates: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OpSpecification {
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "SpecificationCode", skip_serializing_if = "Option::is_none")]
	pub op_specification_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OpOperation {
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
	pub co_change_time: Option<String>,
	#[serde(rename = "OperationType", skip_serializing_if = "Option::is_none")]
	pub op_operation_type: Option<String>,
	#[serde(rename = "ProposalData", skip_serializing_if = "Option::is_none")]
	pub op_proposal_data: Option<Vec<OpProposalData>>,
	#[serde(rename = "Cutting", skip_serializing_if = "Option::is_none")]
	pub op_cutting: Option<Vec<OpCutting>>,
	#[serde(rename = "Specifications", skip_serializing_if = "Option::is_none")]
	pub op_specifications: Option<Vec<OpSpecifications>>,
	#[serde(rename = "CompletionData", skip_serializing_if = "Option::is_none")]
	pub op_completion_data: Option<OpCompletionData>,
	#[serde(rename = "OperationInfo", skip_serializing_if = "Option::is_none")]
	pub op_operation_info: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OpSpecifications {
	#[serde(rename = "Specification", skip_serializing_if = "Option::is_none")]
	pub op_specification: Option<Vec<OpSpecification>>,
}

#[derive(Serialize, Deserialize)]
pub struct StIdentifier {
	#[serde(rename = "IdentifierType", skip_serializing_if = "Option::is_none")]
	pub co_identifier_type: Option<String>,
	#[serde(rename = "IdentifierValue", skip_serializing_if = "Option::is_none")]
	pub co_identifier_value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StSpecialFeatures {
	#[serde(rename = "SpecialFeature", skip_serializing_if = "Option::is_none")]
	pub st_special_feature: Option<Vec<StSpecialFeature>>,
}

#[derive(Serialize, Deserialize)]
pub struct OpCompletionData {
	#[serde(rename = "CompletionDate", skip_serializing_if = "Option::is_none")]
	pub op_completion_date: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ForestPropertyData {
	#[serde(rename = "RealEstates", skip_serializing_if = "Option::is_none")]
	pub re_real_estates: Option<ReRealEstates>,
}

#[derive(Serialize, Deserialize)]
pub struct TsTreeStandDataDate {
	#[serde(rename = "TreeStrata", skip_serializing_if = "Option::is_none")]
	pub tst_tree_strata: Option<Vec<TstTreeStrata>>,
	#[serde(rename = "TreeStandSummary", skip_serializing_if = "Option::is_none")]
	pub tss_tree_stand_summary: Option<Vec<TssTreeStandSummary>>,
	#[serde(rename = "DeadTreeStrata", skip_serializing_if = "Option::is_none")]
	pub dts_dead_tree_strata: Option<Vec<DtsDeadTreeStrata>>,
}

#[derive(Serialize, Deserialize)]
pub struct OpAssortments {
	#[serde(rename = "Assortment", skip_serializing_if = "Option::is_none")]
	pub op_assortment: Option<Vec<OpAssortment>>,
}

#[derive(Serialize, Deserialize)]
pub struct TssTreeStandSummary {
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

#[derive(Serialize, Deserialize)]
pub struct OpAssortment {
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "TreeSpecies", skip_serializing_if = "Option::is_none")]
	pub op_tree_species: Option<String>,
	#[serde(rename = "StemType", skip_serializing_if = "Option::is_none")]
	pub op_stem_type: Option<String>,
	#[serde(rename = "AssortmentVolume", skip_serializing_if = "Option::is_none")]
	pub op_assortment_volume: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StStand {
	#[serde(rename = "StandBasicData", skip_serializing_if = "Option::is_none")]
	pub st_stand_basic_data: Option<Vec<StStandBasicData>>,
	#[serde(rename = "TreeStandData", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data: Option<Vec<TsTreeStandData>>,
	#[serde(rename = "Operations", skip_serializing_if = "Option::is_none")]
	pub op_operations: Option<Vec<OpOperations>>,
	#[serde(rename = "SpecialFeatures", skip_serializing_if = "Option::is_none")]
	pub st_special_features: Option<Vec<StSpecialFeatures>>,
}

#[derive(Serialize, Deserialize)]
pub struct DtsDeadTreeStratum {
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

#[derive(Serialize, Deserialize)]
pub struct StIdentifiers {
	#[serde(rename = "Identifier", skip_serializing_if = "Option::is_none")]
	pub st_identifier: Option<Vec<StIdentifier>>,
}

#[derive(Serialize, Deserialize)]
pub struct ReParcels {
	#[serde(rename = "Parcel", skip_serializing_if = "Option::is_none")]
	pub re_parcel: Option<Vec<ReParcel>>,
}

#[derive(Serialize, Deserialize)]
pub struct TstTreeStratum {
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

#[derive(Serialize, Deserialize)]
pub struct StStands {
	#[serde(rename = "Stand", skip_serializing_if = "Option::is_none")]
	pub st_stand: Option<Vec<StStand>>,
}

#[derive(Serialize, Deserialize)]
pub struct GmlpointProperty {
	#[serde(rename = "Point", skip_serializing_if = "Option::is_none")]
	pub gml_point: Option<Vec<GmlPoint>>,
}

#[derive(Serialize, Deserialize)]
pub struct Gmlexterior {
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<Vec<GmlLinearRing>>,
}

#[derive(Serialize, Deserialize)]
pub struct TstTreeStrata {
	#[serde(rename = "TreeStratum", skip_serializing_if = "Option::is_none")]
	pub tst_tree_stratum: Option<Vec<TstTreeStratum>>,
}

#[derive(Serialize, Deserialize)]
pub struct StStandBasicData {
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "ChangeTime", skip_serializing_if = "Option::is_none")]
	pub co_change_time: Option<String>,
	#[serde(rename = "CompleteState", skip_serializing_if = "Option::is_none")]
	pub st_complete_state: Option<String>,
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
	pub gdt_polygon_geometry: Option<Vec<GdtPolygonGeometry>>,
	#[serde(rename = "StandInfo", skip_serializing_if = "Option::is_none")]
	pub st_stand_info: Option<String>,
	#[serde(rename = "AreaDecrease", skip_serializing_if = "Option::is_none")]
	pub st_area_decrease: Option<String>,
	#[serde(rename = "DitchingYear", skip_serializing_if = "Option::is_none")]
	pub st_ditching_year: Option<String>,
	#[serde(rename = "StandNumberExtension", skip_serializing_if = "Option::is_none")]
	pub st_stand_number_extension: Option<String>,
	#[serde(rename = "Identifiers", skip_serializing_if = "Option::is_none")]
	pub st_identifiers: Option<Vec<StIdentifiers>>,
	#[serde(rename = "CuttingRestriction", skip_serializing_if = "Option::is_none")]
	pub st_cutting_restriction: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TsTreeStandData {
	#[serde(rename = "TreeStandDataDate", skip_serializing_if = "Option::is_none")]
	pub ts_tree_stand_data_date: Option<Vec<TsTreeStandDataDate>>,
}

#[derive(Serialize, Deserialize)]
pub struct OpOperations {
	#[serde(rename = "Operation", skip_serializing_if = "Option::is_none")]
	pub op_operation: Option<Vec<OpOperation>>,
}

#[derive(Serialize, Deserialize)]
pub struct GmlpolygonProperty {
	#[serde(rename = "Polygon", skip_serializing_if = "Option::is_none")]
	pub gml_polygon: Option<Vec<GmlPolygon>>,
}

#[derive(Serialize, Deserialize)]
pub struct StSpecialFeature {
	#[serde(rename = "ChangeState", skip_serializing_if = "Option::is_none")]
	pub co_change_state: Option<String>,
	#[serde(rename = "FeatureCode", skip_serializing_if = "Option::is_none")]
	pub sf_feature_code: Option<String>,
	#[serde(rename = "FeatureAdditionalCode", skip_serializing_if = "Option::is_none")]
	pub sf_feature_additional_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReRealEstate {
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

#[derive(Serialize, Deserialize)]
pub struct OpProposalData {
	#[serde(rename = "ProposalType", skip_serializing_if = "Option::is_none")]
	pub op_proposal_type: Option<String>,
	#[serde(rename = "ProposalYear", skip_serializing_if = "Option::is_none")]
	pub op_proposal_year: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ReParcel {
	#[serde(rename = "ParcelNumber", skip_serializing_if = "Option::is_none")]
	pub re_parcel_number: Option<String>,
	#[serde(rename = "Stands", skip_serializing_if = "Option::is_none")]
	pub st_stands: Option<Vec<StStands>>,
}

#[derive(Serialize, Deserialize)]
pub struct OpCutting {
	#[serde(rename = "CuttingVolume", skip_serializing_if = "Option::is_none")]
	pub op_cutting_volume: Option<String>,
	#[serde(rename = "Assortments", skip_serializing_if = "Option::is_none")]
	pub op_assortments: Option<Vec<OpAssortments>>,
}

#[derive(Serialize, Deserialize)]
pub struct ReRealEstates {
	#[serde(rename = "RealEstate", skip_serializing_if = "Option::is_none")]
	pub re_real_estate: Option<ReRealEstate>,
}

#[derive(Serialize, Deserialize)]
pub struct DtsDeadTreeStrata {
	#[serde(rename = "DeadTreeStratum", skip_serializing_if = "Option::is_none")]
	pub dts_dead_tree_stratum: Option<Vec<DtsDeadTreeStratum>>,
}

#[derive(Serialize, Deserialize)]
pub struct Gmlinterior {
	#[serde(rename = "LinearRing", skip_serializing_if = "Option::is_none")]
	pub gml_linear_ring: Option<Vec<GmlLinearRing>>,
}

#[derive(Serialize, Deserialize)]
pub struct GmlPoint {
	#[serde(rename = "coordinates", skip_serializing_if = "Option::is_none")]
	pub gml_coordinates: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GmlPolygon {
	#[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
	pub gml_exterior: Option<Vec<Gmlexterior>>,
	#[serde(rename = "interior", skip_serializing_if = "Option::is_none")]
	pub gml_interior: Option<Vec<Gmlinterior>>,
}

#[derive(Serialize, Deserialize)]
pub struct GdtPolygonGeometry {
	#[serde(rename = "pointProperty", skip_serializing_if = "Option::is_none")]
	pub gml_point_property: Option<Vec<GmlpointProperty>>,
	#[serde(rename = "polygonProperty", skip_serializing_if = "Option::is_none")]
	pub gml_polygon_property: Option<Vec<GmlpolygonProperty>>,
}

