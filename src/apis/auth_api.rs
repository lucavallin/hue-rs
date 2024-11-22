use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`authenticate`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthenticateError {
    Status401(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// Authenticate to retrieve the HUE application key. Requires to go and press the button on the bridge
pub async fn authenticate(
    configuration: &configuration::Configuration,
    authenticate_request: Option<models::AuthenticateRequest>,
) -> Result<Vec<models::ResponseInner>, Error<AuthenticateError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&authenticate_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthenticateError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
