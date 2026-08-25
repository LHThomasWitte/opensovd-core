use serde::{Deserialize, Serialize};

use crate::{GenericError, types::SupportedTags};

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

pub struct AvailableBulkDataCategories {
    pub items: Vec<BulkDataCategory>,
}

pub struct BulkDataMetadata {
    pub items: Vec<BulkDataDescriptor>,
}

pub struct BulkDataDownload {
    pub signature: String,
    pub data: Vec<u8>,
}

pub struct BulkDataUploadRequest {
    pub signature: String,
    pub data: Vec<u8>,
}

pub struct BulkDataUploadResponse {
    pub id: String,
}

pub struct DeleteBulkDataResult {
    pub deleted_ids: Vec<String>,
    pub errors: Vec<GenericError>,
}
