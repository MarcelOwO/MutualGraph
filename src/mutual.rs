use std::io::Error;

use serde::{Deserialize, Serialize};
use vrchatapi::{apis::configuration, models};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GetMutualsError {
    Status401(models::Error),
    UnknownValue(),
}

pub async fn get_mutuals(
    configuration: &configuration::Configuration,
    user_id: String,
) -> Result<Vec<models::LimitedUserFriend>, Error<GetMutualsError>> {
    let local_config = configuration;
    let client = local_config.client;

    let uri = format!(
        "{}/users/{}/mutuals/friends",
        local_config.base_path, user_id,
    );

    let mut local_var_req_builder = client.request(reqwest::Method::GET, uri.as_str());

    if let Some(ref local_user_agent) = local_config.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_user_agent.clone());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetFriendsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }

    let local_req = match local_req_builder.build() {
        Ok(result) => {}
        Err(e) => {
            println!("Error in building request: {}", e);
        }
    };

    let mutual = vec!["a", "b"];

    Ok(mutual)
}
