use std::io::Error;

use reqwest;
use serde::{Deserialize, Serialize};
use vrchatapi::{
    apis::{ResponseContent, configuration},
    models,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GetMutualsError {
    Status401(models::Error),
    UnknownValue(),
}

pub async fn get_mutuals(
    configuration: &configuration::Configuration,
    user_id: &str,
) -> Result<(), Error<GetMutualsError>> {
    let local_config = configuration;
    let client = local_config.client;

    let uri = format!(
        "{}/users/{}/mutuals/friends",
        local_config.base_path, user_id,
    );

    let mut local_req_builder = client.request(reqwest::Method::GET, uri.as_str());

    if let Some(ref local_user_agent) = local_config.user_agent {
        local_req_builder =
            local_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
    };

    let local_req = local_req_builder.build()?;
}
