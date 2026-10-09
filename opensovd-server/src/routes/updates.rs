// SPDX-FileCopyrightText: Copyright (c) 2026 Contributors to the Eclipse Foundation
// SPDX-License-Identifier: Apache-2.0

use std::{ops::Deref, sync::Arc};

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, put},
};
use axum_extra::extract::{Query, WithRejection};
use http::{HeaderValue, request::Parts};
use opensovd_core::{Topology, UpdateError};
use opensovd_models::{
    Response,
    updates::{
        AvailableUpdates, Phase, Status, UpdateDetail, UpdateDetailQuery, UpdateOrigins,
        UpdateStatus,
    },
};

use super::error::Result;
use crate::routes::{AppState, error::Error};
use crate::schema::JsonSchema;

pub fn routes<V>() -> Router<AppState<V>>
where
    V: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/updates", get(list_updates).post(create_updates))
        .route(
            "/updates/{update-package-id}",
            get(get_update_package).delete(delete_update_package),
        )
        .route(
            "/updates/{update-package-id}/status",
            get(update_package_status),
        )
        .route(
            "/updates/{update-package-id}/automated",
            put(update_package_automated),
        )
        .route(
            "/updates/{update-package-id}/execute",
            put(update_package_execute),
        )
        .route(
            "/updates/{update-package-id}/prepare",
            put(update_package_prepare),
        )
}

async fn list_updates(
    State(topology): State<Topology>,
) -> Result<Json<Response<AvailableUpdates>>> {
    let topo = topology.read().await;
    Ok(Json(Response {
        data: AvailableUpdates {
            items: topo
                .get_updates()
                .ok_or(UpdateError::UpdateProviderNotConfigured)?
                .available()
                .await
                .iter()
                .map(|u| u.id().clone())
                .collect(),
        },
        schema: None,
    }))
}

async fn create_updates(
    State(topology): State<Topology>,
    parts: Parts,
    Json(body): Json<UpdateDetail>,
) -> Result<(StatusCode, HeaderMap)> {
    let topo = topology.read().await;
    let id = topo
        .get_updates()
        .ok_or(UpdateError::UpdateProviderNotConfigured)?
        .push(&body)
        .await?;
    let versioned_uri = super::versioned_uri(&parts);

    let mut headers = HeaderMap::new();
    let location = HeaderValue::from_str(&format!(
        "{versioned_uri}/updates/{}",
        super::entities::encode_path_segment(&id)
    ))
    .map_err(|e| UpdateError::ResponseFailed(e.to_string()))?;
    headers.insert("Location", location);

    Ok((StatusCode::CREATED, headers))
}

async fn get_update_package(
    axum::extract::Path(update_package_id): axum::extract::Path<String>,
    State(topology): State<Topology>,
    WithRejection(Query(query), _): WithRejection<Query<UpdateDetailQuery>, Error>,
) -> Result<Json<Response<UpdateDetail>>> {
    topology
        .read()
        .await
        .get_updates()
        .ok_or(UpdateError::UpdateProviderNotConfigured)?
        .find(&update_package_id)
        .await
        .map_or_else(
            || Err(Error::EntityNotFound(update_package_id)),
            |update| {
                Ok(Json(Response {
                    data: UpdateDetail {
                        id: Some(update.id()),
                        update_name: update.update_name(),
                        automated: update.automated(),
                        origin: update.origin().as_ref().map(|origins| {
                            origins
                                .iter()
                                .filter_map(|o| match o.as_str() {
                                    "remote" => Some(UpdateOrigins::Remote),
                                    "proximity" => Some(UpdateOrigins::Proximity),
                                    _ => None,
                                })
                                .collect()
                        }),
                        update_translation_id: update.update_translation_id(),
                        notes: update.notes(),
                        notes_translation_id: update.notes_translation_id(),
                        user_activity: update.user_activity(),
                        user_activity_translation_id: update.user_activity_translation_id(),
                        preconditions: update.preconditions(),
                        preconditions_translation_id: update.preconditions_translation_id(),
                        execution_conditions: update.execution_conditions(),
                        duration: update.duration(),
                        size: update.size(),
                        updated_components: update.updated_components(),
                        affected_components: update.affected_components(),
                        // the following fields are not included in the response
                        authentication: None,
                        authentication_token: None,
                        targets: vec![],
                    },
                    schema: query.include_schema.then_some(UpdateDetail::schema()),
                }))
            },
        )
}

async fn delete_update_package(
    axum::extract::Path(update_package_id): axum::extract::Path<String>,
    State(topology): State<Topology>,
) -> Result<StatusCode> {
    let updates = topology
        .read()
        .await
        .get_updates()
        .ok_or(UpdateError::UpdateProviderNotConfigured)?;
    // if the update's status is InProgress, we cannot delete it
    if updates
        .feedback(&update_package_id)
        .await
        .is_some_and(|feedback: Arc<UpdateStatus>| feedback.status == Status::InProgress)
    {
        return Ok(StatusCode::METHOD_NOT_ALLOWED);
    }

    updates.remove(&update_package_id).await;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_package_status(
    axum::extract::Path(update_package_id): axum::extract::Path<String>,
    State(topology): State<Topology>,
) -> Result<Json<Response<UpdateStatus>>> {
    let updates = topology
        .read()
        .await
        .get_updates()
        .ok_or(UpdateError::UpdateProviderNotConfigured)?;
    updates
        .feedback(&update_package_id)
        .await
        .map(|feedback: Arc<UpdateStatus>| {
            Json(Response {
                data: feedback.deref().clone(),
                schema: Some(UpdateStatus::schema()),
            })
        })
        .ok_or_else(|| Error::EntityNotFound(update_package_id.clone()))
}

async fn update_package_automated(
    axum::extract::Path(update_package_id): axum::extract::Path<String>,
    parts: Parts,
    State(topology): State<Topology>,
) -> Result<(StatusCode, HeaderMap)> {
    let updates = topology
        .read()
        .await
        .get_updates()
        .ok_or(UpdateError::UpdateProviderNotConfigured)?;
    if let Some(id) = updates
        .all_feedback::<UpdateStatus>()
        .await
        .into_iter()
        .find_map(|(id, feedback)| {
            feedback
                .as_ref()
                .filter(|feedback| {
                    feedback.phase == Phase::Execute && feedback.status == Status::InProgress
                })
                .map(|_| id.clone())
        })
    {
        return Err(UpdateError::UpdateExecutionInProgress(id).into());
    }

    let update = updates
        .find(&update_package_id)
        .await
        .ok_or_else(|| Error::EntityNotFound(update_package_id.clone()))?;

    if update.automated() != Some(true) {
        return Err(UpdateError::AutomatedUpdateNotSupported.into());
    }

    let feedback = updates.feedback_sender(&update_package_id).await;

    updates
        .provider()
        .await
        .ok_or(UpdateError::UpdateProviderNotConfigured)?
        .automated(&*update, feedback)?;

    let versioned_uri = super::versioned_uri(&parts);

    let mut headers = HeaderMap::new();
    let location = HeaderValue::from_str(&format!(
        "{versioned_uri}/updates/{update_package_id}/status"
    ))
    .map_err(|e| UpdateError::ResponseFailed(e.to_string()))?;
    headers.insert("Location", location);

    Ok((StatusCode::ACCEPTED, headers))
}

async fn update_package_execute(
    axum::extract::Path(update_package_id): axum::extract::Path<String>,
    parts: Parts,
    State(topology): State<Topology>,
) -> Result<(StatusCode, HeaderMap)> {
    let updates = topology
        .read()
        .await
        .get_updates()
        .ok_or(UpdateError::UpdateProviderNotConfigured)?;
    if let Some(id) = updates
        .all_feedback::<UpdateStatus>()
        .await
        .into_iter()
        .find_map(|(id, feedback)| {
            feedback
                .as_ref()
                .filter(|feedback| {
                    feedback.phase == Phase::Execute && feedback.status == Status::InProgress
                })
                .map(|_| id.clone())
        })
    {
        return Err(UpdateError::UpdateExecutionInProgress(id).into());
    }

    let update = updates
        .find(&update_package_id)
        .await
        .ok_or_else(|| Error::EntityNotFound(update_package_id.clone()))?;

    let feedback = updates.feedback_sender(&update_package_id).await;

    updates
        .provider()
        .await
        .ok_or(UpdateError::UpdateProviderNotConfigured)?
        .execute(&*update, feedback)?;

    let versioned_uri = super::versioned_uri(&parts);

    let mut headers = HeaderMap::new();
    let location = HeaderValue::from_str(&format!(
        "{versioned_uri}/updates/{update_package_id}/status"
    ))
    .map_err(|e| UpdateError::ResponseFailed(e.to_string()))?;
    headers.insert("Location", location);

    Ok((StatusCode::ACCEPTED, headers))
}

async fn update_package_prepare(
    axum::extract::Path(update_package_id): axum::extract::Path<String>,
    parts: Parts,
    State(topology): State<Topology>,
) -> Result<(StatusCode, HeaderMap)> {
    let updates = topology
        .read()
        .await
        .get_updates()
        .ok_or(UpdateError::UpdateProviderNotConfigured)?;
    let update = updates
        .find(&update_package_id)
        .await
        .ok_or_else(|| Error::EntityNotFound(update_package_id.clone()))?;

    let feedback = updates.feedback_sender(&update_package_id).await;

    updates
        .provider()
        .await
        .ok_or(UpdateError::UpdateProviderNotConfigured)?
        .prepare(&*update, feedback)?;

    let versioned_uri = super::versioned_uri(&parts);

    let mut headers = HeaderMap::new();
    let location = HeaderValue::from_str(&format!(
        "{versioned_uri}/updates/{update_package_id}/status"
    ))
    .map_err(|e| UpdateError::ResponseFailed(e.to_string()))?;
    headers.insert("Location", location);

    Ok((StatusCode::ACCEPTED, headers))
}
