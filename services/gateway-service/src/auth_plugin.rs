use std::{ops::ControlFlow, sync::Arc};

use apollo_router::{
  graphql,
  layers::ServiceBuilderExt,
  plugin::{Plugin, PluginInit},
  services::supergraph
};
use async_trait::async_trait;
use http::StatusCode;
use schemars::JsonSchema;
use serde::Deserialize;
use tower::{BoxError, ServiceBuilder, ServiceExt};
use uuid::Uuid;

const USER_CONTEXT_PARAM_NAME: &str = "user_id";
const API_TOKEN_CONTEXT_PARAM_NAME: &str = "api_token";
const AUTHORIZATION_HEADER_NAME: &str = "authorization";
const USER_HEADER_NAME: &str = "dottit-user-id";
const API_TOKEN_HEADER_NAME: &str = "dottit-api-token";

pub struct AuthPlugin {
  state: Arc<AuthPluginState>
}

pub struct AuthPluginState {
  http_client: reqwest::Client,
  auth_server: String
}

#[derive(Deserialize, JsonSchema)]
pub struct AuthPluginConfig {
  auth_server: String
}

#[derive(Deserialize)]
struct ValidateResponse {
  user_id: Uuid
}

impl AuthPluginState {
  async fn validate_api_token(&self, token: &str) -> Result<ValidateResponse, BoxError> {
    let response = self
      .http_client
      .get(format!("{}/validate/{token}", self.auth_server))
      .send()
      .await?
      .json::<ValidateResponse>()
      .await?;

    Ok(response)
  }
}

#[async_trait]
impl Plugin for AuthPlugin {
  type Config = AuthPluginConfig;

  async fn new(init: PluginInit<Self::Config>) -> Result<Self, BoxError> {
    Ok(Self {
      state: Arc::new(AuthPluginState {
        http_client: reqwest::Client::new(),
        auth_server: init.config.auth_server
      })
    })
  }

  fn supergraph_service(&self, service: supergraph::BoxService) -> supergraph::BoxService {
    let tmp_state = self.state.clone();
    let handler = move |request: supergraph::Request| {
      let state = tmp_state.clone();
      async move {
        let headers = request.supergraph_request.headers();
        let auth_header_value = headers.get(AUTHORIZATION_HEADER_NAME);

        let token = match auth_header_value {
          Some(auth_header_value) => {
            let auth_header_value_str = auth_header_value.to_str()?;
            match get_auth_token_from_header_value(auth_header_value_str) {
              Some(token) => token,
              None => {
                let error_message = "invalid API token format".to_string();
                let response = supergraph::Response::error_builder()
                  .error(graphql::Error::builder().message(error_message).build())
                  .status_code(StatusCode::BAD_REQUEST)
                  .context(request.context)
                  .build()?;
                return Ok(ControlFlow::Break(response));
              }
            }
          }
          None => return Ok(ControlFlow::Continue(request))
        };

        match state.validate_api_token(token).await {
          Ok(auth) => {
            request
              .context
              .insert(USER_CONTEXT_PARAM_NAME, auth.user_id)?;
            request
              .context
              .insert(API_TOKEN_CONTEXT_PARAM_NAME, token.to_owned())?;

            Ok(ControlFlow::Continue(request))
          }
          Err(e) => {
            let error_message = format!("failed to validate API token: {e}");
            let response = supergraph::Response::error_builder()
              .error(graphql::Error::builder().message(error_message).build())
              .status_code(StatusCode::BAD_REQUEST)
              .context(request.context)
              .build()?;
            Ok(ControlFlow::Break(response))
          }
        }
      }
    };

    let request_mapper = |mut request: supergraph::Request| {
      let maybe_user_role: Option<String> = request.context.get(USER_CONTEXT_PARAM_NAME).unwrap();
      let maybe_api_token: Option<String> =
        request.context.get(API_TOKEN_CONTEXT_PARAM_NAME).unwrap();

      if let Some(user_role) = maybe_user_role {
        request
          .supergraph_request
          .headers_mut()
          .insert(USER_HEADER_NAME, user_role.try_into().unwrap());
      } else {
        request
          .supergraph_request
          .headers_mut()
          .remove(USER_HEADER_NAME);
      }

      if let Some(api_token) = maybe_api_token {
        request
          .supergraph_request
          .headers_mut()
          .insert(API_TOKEN_HEADER_NAME, api_token.try_into().unwrap());
      } else {
        request
          .supergraph_request
          .headers_mut()
          .remove(API_TOKEN_HEADER_NAME);
      }

      request
    };

    ServiceBuilder::new()
      .checkpoint_async(handler)
      .map_request(request_mapper)
      .buffered()
      .service(service)
      .boxed()
  }
}

fn get_auth_token_from_header_value(auth_header_value: &str) -> Option<&str> {
  let bearer = "Bearer ";
  auth_header_value.strip_prefix(bearer)
}
