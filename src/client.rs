use std::{rc::Rc, sync::Arc};

use crate::sendable::Sendable;
use http_client::{http_types::Headers, HttpClient, Request, Response};
use serde_json::error;
// use reqwest::{
//     header::{HeaderMap, HeaderValue},
//     ClientBuilder, RequestBuilder, Response,
// };
use thiserror::Error;

/// ClientError provides a mechanism to determine when the access token has expired. All other
/// errors will be encapsulated by UnknownError.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Invalid Access Token")]
    InvalidToken,
    #[error("HTTP Error: {0}")]
    HttpError(http_client::Error),
    #[error("Unknown Error: {0}")]
    UnknownError(String),
}

// impl From<davisjr::errors::Error> for ClientError {
//     fn from(value: davisjr::errors::Error) -> Self {
//         Self::UnknownError(value.to_string())
//     }
// }

impl From<anyhow::Error> for ClientError {
    fn from(value: anyhow::Error) -> Self {
        Self::UnknownError(value.to_string())
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(value: serde_json::Error) -> Self {
        Self::UnknownError(value.to_string())
    }
}

impl From<url::ParseError> for ClientError {
    fn from(value: url::ParseError) -> Self {
        Self::UnknownError(value.to_string())
    }
}

impl From<http_client::Error> for ClientError {
    fn from(value: http_client::Error) -> Self {
        Self::HttpError(value)
    }
}

// impl From<reqwest::Error> for ClientError {
//     fn from(value: reqwest::Error) -> Self {
//         Self::UnknownError(value.to_string())
//     }
// }

// impl From<reqwest::header::ToStrError> for ClientError {
//     fn from(value: reqwest::header::ToStrError) -> Self {
//         Self::UnknownError(value.to_string())
//     }
// }

/// Client is a Google Calendar client. The access key must have already been fetched and the oauth
/// negotiation should have already been completed. The client itself only implements HTTP verbs
/// that accept Sendable implementations. You must use the decorated clients such as EventClient
/// and CalendarListClient to do transactional work.
#[derive(Debug)]
pub struct GCalClient<C> {
    // client: reqwest::Client,
    client: Arc<C>,
    access_key: String,
    headers: Option<Headers>,
    debug: bool,
}

impl<C> Clone for GCalClient<C> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            access_key: self.access_key.clone(),
            headers: self.headers.clone(),
            debug: self.debug,
        }
    }
}

impl<C> GCalClient<C> {
    /// Create a new client. Requires an access key.
    pub fn new(client: C, access_key: String) -> Result<Self, ClientError> {
        Ok(Self {
            client: Arc::new(client),
            access_key,
            headers: None,
            debug: false,
        })
    }

    pub fn set_debug(&mut self) {
        self.debug = true
    }

    fn set_bearer(&self, mut req: Request) -> Request {
        req.insert_header("Authorization", format!("Bearer {}", self.access_key));
        req
    }
    fn get_url(
        &self,
        method: &str,
        target: &impl Sendable,
        action: Option<String>,
    ) -> Result<url::Url, ClientError> {
        let url = target.url(action)?;

        if self.debug {
            let byt = target.body_bytes()?;
            eprintln!(
                "[{}] {} | {}",
                method,
                url,
                String::from_utf8(byt).unwrap_or_default()
            );
        }

        Ok(url)
    }
}

impl<C: HttpClient> GCalClient<C> {
    async fn send(&self, mut req: Request) -> Result<Response, ClientError> {
        for (name, val) in self.headers.iter().flatten() {
            req.insert_header(name, val);
        }
        req = self.set_bearer(req);
        let resp = self
            .client
            .send(req)
            .await
            .map_err(|e| ClientError::UnknownError(e.to_string()))?;
        if resp.status() != 200 {
            if let Some(header) = resp.header("WWW-Authenticate") {
                if header
                    .as_str()
                    .starts_with(r#"Bearer error="invalid_token""#)
                {
                    return Err(ClientError::InvalidToken);
                }
            }
            Ok(resp)
        } else {
            Ok(resp)
        }
    }

    /// Perform a GET request.
    pub async fn get(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        // self.client.get(self.get_url("GET", &target, action)?)
        let req = Request::get(self.get_url("GET", &target, action)?);
        self.send(req).await
    }

    /// Perform a POST request.
    pub async fn post(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        let mut req = Request::post(self.get_url("POST", &target, action)?);
        req.set_body(target.body_bytes()?);
        self.send(req).await
    }

    /// Perform a PUT request.
    pub async fn put(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        let mut req = Request::put(self.get_url("PUT", &target, action)?);
        req.set_body(target.body_bytes()?);
        self.send(req).await
    }

    /// Perform a PATCH request.
    pub async fn patch(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        let mut req = Request::patch(self.get_url("PATCH", &target, action)?);
        req.set_body(target.body_bytes()?);
        self.send(req).await
    }

    /// Perform a DELETE request.
    pub async fn delete(
        &self,
        action: Option<String>,
        target: impl Sendable,
    ) -> Result<Response, ClientError> {
        let req = Request::delete(self.get_url("DELETE", &target, action)?);
        self.send(req).await
    }
}
