// SPDX-FileCopyrightText: Copyright (c) 2026 Contributors to the Eclipse Foundation
// SPDX-License-Identifier: Apache-2.0

//! Bulk-Data provider trait and types.

use async_trait::async_trait;

use crate::{CategoryInfo, DataError};

pub struct Metadata {}

pub struct BulkData {}

/// A `Result` alias where the `Err` variant is [`DataError`].
pub type Result<T> = std::result::Result<T, DataError>;

#[async_trait]
pub trait BulkDataProvider: Send + Sync + 'static {
    async fn categories(&self) -> Result<Vec<CategoryInfo>>;

    async fn list(&self, category_id: &str) -> Result<Vec<Metadata>>;

    async fn download(&self, data_id: &str, include_schema: bool) -> Result<BulkData>;

    async fn upload(&self, data_id: &str, data: BulkData) -> Result<()>;

    async fn delete(&self, data_id: &str) -> Result<()>;

    async fn delete_category(&self, category_id: &str) -> Result<()>;
}
