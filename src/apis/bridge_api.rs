use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`get_bridge`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBridgeError {
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    Status507(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_bridges`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetBridgesError {
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    Status507(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_bridge`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateBridgeError {
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status405(models::ErrorResponse),
    Status406(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    Status507(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Get details of a single bridge from its given `{bridgeId}`.
pub async fn get_bridge(
    configuration: &configuration::Configuration,
    bridge_id: &str,
) -> Result<models::GetBridges200Response, Error<GetBridgeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clip/v2/resource/bridge/{bridgeId}",
        local_var_configuration.base_path,
        bridgeId = crate::apis::urlencode(bridge_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder =
            local_var_req_builder.header("hue-application-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetBridgeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all available bridges
pub async fn get_bridges(
    configuration: &configuration::Configuration,
) -> Result<models::GetBridges200Response, Error<GetBridgesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clip/v2/resource/bridge",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder =
            local_var_req_builder.header("hue-application-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetBridgesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a single bridge from its given `{bridgeId}`.
pub async fn update_bridge(
    configuration: &configuration::Configuration,
    bridge_id: &str,
    bridge_put: Option<models::BridgePut>,
) -> Result<models::UpdateDevice200Response, Error<UpdateBridgeError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clip/v2/resource/bridge/{bridgeId}",
        local_var_configuration.base_path,
        bridgeId = crate::apis::urlencode(bridge_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder =
            local_var_req_builder.header("hue-application-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&bridge_put);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateBridgeError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
