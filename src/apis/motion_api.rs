use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`get_motion_sensor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMotionSensorError {
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

/// struct for typed errors of method [`get_motion_sensors`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetMotionSensorsError {
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

/// struct for typed errors of method [`update_motion_sensor`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateMotionSensorError {
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

/// Get details of a single motion sensor from its given `{motionId}`.
pub async fn get_motion_sensor(
    configuration: &configuration::Configuration,
    motion_id: &str,
) -> Result<models::GetMotionSensors200Response, Error<GetMotionSensorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clip/v2/resource/motion/{motionId}",
        local_var_configuration.base_path,
        motionId = crate::apis::urlencode(motion_id)
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
        let local_var_entity: Option<GetMotionSensorError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all available motion sensors.
pub async fn get_motion_sensors(
    configuration: &configuration::Configuration,
) -> Result<models::GetMotionSensors200Response, Error<GetMotionSensorsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clip/v2/resource/motion",
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
        let local_var_entity: Option<GetMotionSensorsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update a single motion sensor from its given `{motionId}`.
pub async fn update_motion_sensor(
    configuration: &configuration::Configuration,
    motion_id: &str,
    motion_put: Option<models::MotionPut>,
) -> Result<models::UpdateDevice200Response, Error<UpdateMotionSensorError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/clip/v2/resource/motion/{motionId}",
        local_var_configuration.base_path,
        motionId = crate::apis::urlencode(motion_id)
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
    local_var_req_builder = local_var_req_builder.json(&motion_put);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateMotionSensorError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
