use reqwest::{Client, Response, Url};
use serde::de::DeserializeOwned;
use snafu::ResultExt;
#[cfg(feature = "clustering")]
use tokki_common::hmac::HmacForm;

use crate::healthcheck::{HealthcheckRequest, HealthcheckResponse};
#[cfg(feature = "clustering")]
use crate::{
    ApiErrorResponse, ClientError,
    client_error::{JsonParseSnafu, ReqwestSnafu, UrlPathParseSnafu},
    clustering::{ReplicateLogRequest, ReplicateLogResponse},
    get_records::{GetRecordsRequest, GetRecordsResponse},
    put_record::{PutRecordsRequest, PutRecordsResponse},
};

#[derive(Clone)]
pub struct TokkiClient {
    client: Client,
    base_url: Url,
}

impl TokkiClient {
    pub fn new(base_url: Url) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    fn api_url(&self, path: &'static str) -> Result<Url, ClientError> {
        self.base_url
            .join(path)
            .with_context(|_| UrlPathParseSnafu {
                base_url: self.base_url.to_string(),
                path,
            })
    }

    async fn process_json_response<T: DeserializeOwned>(
        &self,
        res: Response,
    ) -> Result<T, ClientError> {
        if res.status().is_success() {
            res.json::<T>().await.with_context(|_| JsonParseSnafu {
                base_url: self.base_url.to_string(),
            })
        } else {
            let response =
                res.json::<ApiErrorResponse>()
                    .await
                    .with_context(|_| JsonParseSnafu {
                        base_url: self.base_url.to_string(),
                    })?;
            Err(ClientError::BadResponse {
                base_url: self.base_url.to_string(),
                response,
            })
        }
    }

    pub async fn get_healthcheck(&self) -> Result<HealthcheckResponse, ClientError> {
        let url = self.api_url("healthcheck")?;
        let req = HealthcheckRequest::new();

        let res = self
            .client
            .get(url)
            .json(&req)
            .send()
            .await
            .with_context(|_| ReqwestSnafu {
                base_url: self.base_url.to_string(),
            })?;

        self.process_json_response(res).await
    }

    pub async fn put_record(
        &self,
        req: PutRecordsRequest,
    ) -> Result<PutRecordsResponse, ClientError> {
        let url = self.api_url("records")?;

        let res = self
            .client
            .put(url)
            .json(&req)
            .send()
            .await
            .with_context(|_| ReqwestSnafu {
                base_url: self.base_url.to_string(),
            })?;

        self.process_json_response(res).await
    }

    pub async fn get_records(
        &self,
        req: GetRecordsRequest,
    ) -> Result<GetRecordsResponse, ClientError> {
        let url = self.api_url("records")?;

        let res = self
            .client
            .get(url)
            .json(&req)
            .send()
            .await
            .with_context(|_| ReqwestSnafu {
                base_url: self.base_url.to_string(),
            })?;

        self.process_json_response(res).await
    }

    #[cfg(feature = "clustering")]
    pub async fn replicate_records(
        &self,
        req: ReplicateLogRequest,
        token: &str,
    ) -> Result<HmacForm<ReplicateLogResponse>, ClientError> {
        use tokki_common::hmac::HmacForm;

        let req = HmacForm::new(req, token);

        let url = self.api_url("replication")?;

        let res = self
            .client
            .get(url)
            .json(&req)
            .send()
            .await
            .with_context(|_| ReqwestSnafu {
                base_url: self.base_url.to_string(),
            })?;

        self.process_json_response(res).await
    }
}
