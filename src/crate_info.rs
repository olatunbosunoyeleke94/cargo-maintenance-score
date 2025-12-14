use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    #[serde(rename = "crate")]
    pub crate_info: CrateInfo,
}

#[derive(Deserialize, Debug)]
pub struct CrateInfo {
    pub updated_at: String,
    pub recent_downloads: Option<u64>,
    pub max_version: String,
}

#[derive(Debug)]
pub struct CrateMaintenance {
    pub days_since_update: i64,
    pub recent_downloads: u64,
    pub max_version: String,
    pub score: u8,
    pub risk_level: String,
}
