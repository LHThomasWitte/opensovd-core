// SPDX-FileCopyrightText: Copyright (c) 2026 Contributors to the Eclipse Foundation
// SPDX-License-Identifier: Apache-2.0

//! Bulk-Data resource endpoints.
//!
//! Provides routes for:
//! - GET /{entity-collection}/{entity-id}/bulk-data - List bulk-data categories
//! - GET /{entity-collection}/{entity-id}/bulk-data/{category} - List of BulkDataDescriptors for a specific category
//! - POST /{entity-collection}/{entity-id}/bulk-data/{category} - Upload bulk data to the SOVD server and create a resource for the bulk data
//! - DELETE /{entity-collection}/{entity-id}/bulk-data/{category} - Delete all bulk data resources for a specific category
//! - GET /{entity-collection}/{entity-id}/bulk-data/{category}/{bulk-data-id} - Download a specific bulk data resource
//! - DELETE /{entity-collection}/{entity-id}/bulk-data/{category}/{bulk-data-id} - Delete a specific bulk data resource

use axum::{
    Router,
    extract::{Path, State},
    routing::get,
};
use opensovd_core::Topology;

use crate::routes::AppState;

pub fn routes<V>() -> Router<AppState<V>>
where
    V: Clone + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/{entity-collection}/{entity-id}/bulk-data",
            get(bulk_data_categories),
        )
        .route(
            "/{entity-collection}/{entity-id}/bulk-data/{category}",
            get(bulk_data_descriptors)
                .post(upload_bulk_data)
                .delete(delete_bulk_data_category),
        )
        .route(
            "/{entity-collection}/{entity-id}/bulk-data/{category}/{bulk-data-id}",
            get(download_bulk_data).delete(delete_bulk_data),
        )
}

async fn bulk_data_categories(
    State(_topology): State<Topology>,
    Path((_entity_collection, _entity_id)): Path<(String, String)>,
) {
    todo!()
}

async fn bulk_data_descriptors(
    State(_topology): State<Topology>,
    Path((_entity_collection, _entity_id, _category)): Path<(String, String, String)>,
) {
    todo!()
}

async fn upload_bulk_data(
    State(_topology): State<Topology>,
    Path((_entity_collection, _entity_id, _category)): Path<(String, String, String)>,
) {
    todo!()
}

async fn delete_bulk_data_category(
    State(_topology): State<Topology>,
    Path((_entity_collection, _entity_id, _category)): Path<(String, String, String)>,
) {
    todo!()
}

async fn download_bulk_data(
    State(_topology): State<Topology>,
    Path((_entity_collection, _entity_id, _category, _bulk_data_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
) {
    todo!()
}

async fn delete_bulk_data(
    State(_topology): State<Topology>,
    Path((_entity_collection, _entity_id, _category, _bulk_data_id)): Path<(
        String,
        String,
        String,
        String,
    )>,
) {
    todo!()
}
