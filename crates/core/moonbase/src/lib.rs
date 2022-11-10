mod api;
mod common;
mod errors;

use crate::common::get_host;
use common::{get_request, parse_response, post_request, Response};
use moon_error::map_io_to_fs_error;
use moon_logger::{color, warn};
use reqwest::multipart::{Form, Part};
use std::fs;
use std::path::Path;

pub use api::*;
pub use errors::MoonbaseError;

const LOG_TARGET: &str = "moonbase";

#[derive(Debug)]
pub struct Moonbase {
    auth_token: String,

    organization_id: i32,

    repository_id: i32,
}

impl Moonbase {
    pub async fn signin(
        secret_key: String,
        api_key: String,
        slug: String,
    ) -> Result<Option<Moonbase>, MoonbaseError> {
        let data = post_request(
            "auth/repository/signin",
            SigninBody {
                organization_key: secret_key,
                repository: slug,
                repository_key: api_key,
            },
            None,
        )
        .await?;

        match data {
            Response::Failure { message, status } => {
                warn!(
                    target: LOG_TARGET,
                    "Failed to sign in to moonbase, please check your API keys. Process will still continue...\nFailure: {} ({})", color::muted_light(message), status
                );

                Ok(None)
            }
            Response::Success(SigninResponse {
                organization_id,
                repository_id,
                token,
            }) => Ok(Some(Moonbase {
                auth_token: token,
                organization_id,
                repository_id,
            })),
        }
    }

    pub async fn get_artifact(&self, hash: &str) -> Result<Option<Artifact>, MoonbaseError> {
        let response = get_request(format!("artifacts/{}", hash), Some(&self.auth_token)).await?;

        match response {
            Response::Success(ArtifactResponse { artifact }) => Ok(Some(artifact)),
            _ => Ok(None),
        }
    }

    pub async fn upload_artifact(
        &self,
        hash: &str,
        target: &str,
        path: &Path,
    ) -> Result<Option<Artifact>, MoonbaseError> {
        let file = fs::read(path).map_err(|e| map_io_to_fs_error(e, path.to_path_buf()))?;
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy().to_string(),
            None => format!("{}.tar.gz", hash),
        };

        let form = Form::new()
            .text("repository", self.repository_id.to_string())
            .text("target", target.to_owned())
            .part("file", Part::bytes(file).file_name(file_name));

        let request = reqwest::Client::new()
            .post(format!("{}/artifacts/{}", get_host(), hash))
            .multipart(form)
            .bearer_auth(&self.auth_token)
            .header("Accept", "application/json");

        let response = request.send().await?;
        let data: Response<ArtifactResponse> = parse_response(response.text().await?)?;

        match data {
            Response::Success(ArtifactResponse { artifact }) => Ok(Some(artifact)),
            _ => Ok(None),
        }
    }
}
