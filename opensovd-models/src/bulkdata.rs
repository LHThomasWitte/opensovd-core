use serde::{Deserialize, Serialize};

use crate::{GenericError, types::SupportedTags};

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BulkDataCategoriesQuery {
    pub include_schema: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BulkDataDescriptorsQuery {
    pub include_schema: bool,
    pub created_before: Option<String>,
    pub created_after: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct BulkDataCategory(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct BulkDataDescriptor {
    pub id: String,
    pub mimetype: String,
    pub name: Option<String>,
    pub translation_id: Option<String>,
    pub size: Option<u64>,
    pub creation_date: Option<String>,
    pub last_modified: Option<String>,
    pub hash: Option<String>,
    pub hash_algorithm: Option<String>,
    pub tags: Option<SupportedTags>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct AvailableBulkDataCategories {
    pub items: Vec<BulkDataCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct BulkDataMetadata {
    pub items: Vec<BulkDataDescriptor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct BulkDataDownload {
    pub signature: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct BulkDataUpload {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "jsonschema", derive(schemars::JsonSchema))]
pub struct DeleteBulkDataResult {
    pub deleted_ids: Vec<String>,
    pub errors: Vec<GenericError>,
}
