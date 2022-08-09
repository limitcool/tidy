use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub versions: Vec<Version>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    #[serde(rename = "audit_actions")]
    pub audit_actions: Vec<AuditAction>,
    #[serde(rename = "crate")]
    pub crate_field: String,
    #[serde(rename = "crate_size")]
    pub crate_size: Option<i64>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dl_path")]
    pub dl_path: String,
    pub downloads: i64,
    pub features: Features,
    pub id: i64,
    pub license: String,
    pub links: Links,
    pub num: String,
    #[serde(rename = "published_by")]
    pub published_by: Option<PublishedBy>,
    #[serde(rename = "readme_path")]
    pub readme_path: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub yanked: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditAction {
    pub action: String,
    pub time: String,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub avatar: String,
    pub id: i64,
    pub login: String,
    pub name: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {
    #[serde(default)]
    pub alloc: Vec<String>,
    #[serde(default)]
    pub default: Vec<String>,
    #[serde(default)]
    pub getrandom: Vec<String>,
    #[serde(rename = "min_const_gen")]
    #[serde(default)]
    pub min_const_gen: Vec<Value>,
    #[serde(default)]
    pub nightly: Vec<String>,
    #[serde(default)]
    pub serde1: Vec<String>,
    #[serde(rename = "simd_support")]
    #[serde(default)]
    pub simd_support: Vec<String>,
    #[serde(rename = "small_rng")]
    #[serde(default)]
    pub small_rng: Vec<String>,
    #[serde(default)]
    pub std: Vec<String>,
    #[serde(rename = "std_rng")]
    #[serde(default)]
    pub std_rng: Vec<String>,
    #[serde(default)]
    pub stdweb: Vec<String>,
    #[serde(rename = "wasm-bindgen")]
    #[serde(default)]
    pub wasm_bindgen: Vec<String>,
    #[serde(rename = "i128_support")]
    #[serde(default)]
    pub i128_support: Vec<Value>,
    #[serde(rename = "serde-1")]
    pub serde_1: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub authors: String,
    pub dependencies: String,
    #[serde(rename = "version_downloads")]
    pub version_downloads: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedBy {
    pub avatar: String,
    pub id: i64,
    pub login: String,
    pub name: String,
    pub url: String,
}
