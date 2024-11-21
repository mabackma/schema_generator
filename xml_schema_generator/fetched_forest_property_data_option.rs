#[derive(Serialize, Deserialize)]
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
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Stands")]
    pub st_stands: StStands,
}

#[derive(Serialize, Deserialize)]
pub struct StStands {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Stand")]
    pub st_stand: Vec<StStand>,
}

#[derive(Serialize, Deserialize)]
pub struct StStand {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SpecialFeatures")]
    pub st_special_features: Option<StSpecialFeatures>,
    #[serde(rename = "StandBasicData")]
    pub st_stand_basic_data: StStandBasicData,
    #[serde(rename = "TreeStandData")]
    pub ts_tree_stand_data: TsTreeStandData,
    #[serde(rename = "Operations")]
    pub op_operations: Option<OpOperations>,
}

#[derive(Serialize, Deserialize)]
pub struct StSpecialFeatures {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "SpecialFeature")]
    pub st_special_feature: Vec<StSpecialFeature>,
}

#[derive(Serialize, Deserialize)]
pub struct StSpecialFeature {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct StStandBasicData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "PolygonGeometry")]
    pub gdt_polygon_geometry: GdtPolygonGeometry,
}

#[derive(Serialize, Deserialize)]
pub struct GdtPolygonGeometry {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "pointProperty")]
    pub gml_point_property: GmlPointProperty,
    #[serde(rename = "polygonProperty")]
    pub gml_polygon_property: GmlPolygonProperty,
}

#[derive(Serialize, Deserialize)]
pub struct GmlPointProperty {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Point")]
    pub gml_point: GmlPoint,
}

#[derive(Serialize, Deserialize)]
pub struct GmlPoint {
    #[serde(rename = "@srsName")]
    pub srs_name: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GmlPolygonProperty {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Polygon")]
    pub gml_polygon: GmlPolygon,
}

#[derive(Serialize, Deserialize)]
pub struct GmlPolygon {
    #[serde(rename = "@srsName")]
    pub srs_name: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TsTreeStandData {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TreeStandDataDate")]
    pub ts_tree_stand_data_date: Vec<TsTreeStandDataDate>,
}

#[derive(Serialize, Deserialize)]
pub struct TsTreeStandDataDate {
    #[serde(rename = "@type")]
    pub ts_tree_stand_data_date_type: Option<String>,
    #[serde(rename = "@date")]
    pub date: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "DeadTreeStrata")]
    pub dts_dead_tree_strata: Option<DtsDeadTreeStrata>,
    #[serde(rename = "TreeStrata")]
    pub tst_tree_strata: Option<TstTreeStrata>,
    #[serde(rename = "TreeStandSummary")]
    pub tss_tree_stand_summary: Option<TssTreeStandSummary>,
}

#[derive(Serialize, Deserialize)]
pub struct DtsDeadTreeStrata {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "DeadTreeStratum")]
    pub dts_dead_tree_stratum: Vec<DtsDeadTreeStratum>,
}

#[derive(Serialize, Deserialize)]
pub struct DtsDeadTreeStratum {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TstTreeStrata {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "TreeStratum")]
    pub tst_tree_stratum: Vec<TstTreeStratum>,
}

#[derive(Serialize, Deserialize)]
pub struct TstTreeStratum {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TssTreeStandSummary {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OpOperations {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Operation")]
    pub op_operation: Vec<OpOperation>,
}

#[derive(Serialize, Deserialize)]
pub struct OpOperation {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@mainType")]
    pub main_type: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Cutting")]
    pub op_cutting: Option<OpCutting>,
    #[serde(rename = "Silviculture")]
    pub op_silviculture: Option<OpSilviculture>,
}

#[derive(Serialize, Deserialize)]
pub struct OpCutting {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Assortments")]
    pub op_assortments: Option<OpAssortments>,
}

#[derive(Serialize, Deserialize)]
pub struct OpAssortments {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Assortment")]
    pub op_assortment: Vec<OpAssortment>,
}

#[derive(Serialize, Deserialize)]
pub struct OpAssortment {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "$text")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OpSilviculture {
}

