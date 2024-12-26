use crate::async_binance::errors::CustomError;
use crate::async_binance::models::{ExchangeInfo, ListenKey};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use ring::hmac;
use std::time::Duration;
use tracing::info;

#[derive(Clone)]
pub struct AsyncBinanceClient {
    api_key: String,
    secret_key: String,
    client_session: reqwest::Client,
    host: String, // Base URL of Spot or Futures
}

impl AsyncBinanceClient {
    pub fn new(
        api_key: Option<String>,
        secret_key: Option<String>,
        host: String,
        timeout: Option<u64>,
    ) -> Self {
        let mut client_builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        if let Some(timeout_sec) = timeout {
            client_builder = client_builder.timeout(Duration::from_secs(timeout_sec))
        }
        AsyncBinanceClient {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            client_session: client_builder.build().unwrap(),
            host,
        }
    }

    fn signed_request(&self, endpoint: &str, request_body: &str) -> String {
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret_key.as_bytes());
        let signature = hex::encode(hmac::sign(&signed_key, request_body.as_bytes()).as_ref());
        format!(
            "{}{}?{}&signature={}",
            self.host, endpoint, request_body, signature
        )
    }

    fn build_headers(&self, content_type: bool) -> std::result::Result<HeaderMap, CustomError> {
        let mut custom_headers = HeaderMap::new();
        custom_headers.insert(USER_AGENT, HeaderValue::from_static("binance-rs"));
        if content_type {
            custom_headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            );
        }
        custom_headers.insert(
            HeaderName::from_static("x-mbx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        Ok(custom_headers)
    }
    async fn handler<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> std::result::Result<T, CustomError> {
        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json().await?),
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err(CustomError::InternalServerError),
            reqwest::StatusCode::SERVICE_UNAVAILABLE => Err(CustomError::ServiceUnavailable),
            reqwest::StatusCode::UNAUTHORIZED => Err(CustomError::Unauthorized),
            reqwest::StatusCode::BAD_REQUEST => {
                let error = response.json().await?;
                Err(CustomError::BinanceError { response: error })
            }
            s => Err(CustomError::Msg(format!("Received response: {s:?}"))),
        }
    }

    pub async fn signed_get<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        request_body: &str,
    ) -> std::result::Result<T, CustomError> {
        let url: String = self.signed_request(endpoint, request_body);
        let response: reqwest::Response = self
            .client_session
            .get(&url)
            .headers(self.build_headers(true)?)
            .send()
            .await?;
        self.handler(response).await
    }

    pub async fn signed_post<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        request_body: &str,
    ) -> std::result::Result<T, CustomError> {
        let url: String = self.signed_request(endpoint, request_body);
        let response: reqwest::Response = self
            .client_session
            .post(&url)
            .headers(self.build_headers(true)?)
            .send()
            .await?;
        self.handler(response).await
    }

    pub async fn signed_put<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        request_body: &str,
    ) -> std::result::Result<T, CustomError> {
        let url: String = self.signed_request(endpoint, request_body);
        let response: reqwest::Response = self
            .client_session
            .put(&url)
            .headers(self.build_headers(true)?)
            .send()
            .await?;
        self.handler(response).await
    }

    pub async fn signed_delete<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        request_body: &str,
    ) -> std::result::Result<T, CustomError> {
        let url: String = self.signed_request(endpoint, request_body);
        let response: reqwest::Response = self
            .client_session
            .delete(&url)
            .headers(self.build_headers(true)?)
            .send()
            .await?;
        self.handler(response).await
    }

    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        request: Option<&str>,
    ) -> Result<T, CustomError> {
        let url = request
            .map(|r| format!("{}{}?{}", self.host, endpoint, r))
            .unwrap_or_else(|| format!("{}{}", self.host, endpoint));

        let response = self.client_session.get(&url).send().await?;

        self.handler(response).await
    }

    pub async fn post<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        symbol: Option<&str>,
    ) -> Result<T, CustomError> {
        let url = symbol
            .map(|s| format!("{}{}?symbol={}", self.host, endpoint, s))
            .unwrap_or_else(|| format!("{}{}", self.host, endpoint));

        let response = self
            .client_session
            .post(url)
            .headers(self.build_headers(false)?)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn put<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        listen_key: &str,
        symbol: Option<&str>,
    ) -> Result<T, CustomError> {
        let data = symbol
            .map(|s| format!("listenKey={listen_key}&symbol={s}"))
            .unwrap_or_else(|| format!("listenKey={listen_key}"));
        let headers = self.build_headers(false)?;
        let url = format!("{}{}?{}", self.host, endpoint, data);
        let response = self
            .client_session
            .put(&url)
            .headers(headers)
            .send()
            .await?;

        self.handler(response).await
    }

    pub async fn get_listen_key(&self) -> Result<String, CustomError> {
        let response: Result<ListenKey, CustomError> = self.post("listenKey", None).await;
        match response {
            Ok(data) => {
                info!("Obtain Listen Key");
                Ok(data.listenKey)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn keep_listen_key_alive(&self, listen_key: &str) -> Result<(), CustomError> {
        let response: Result<ListenKey, CustomError> =
            self.put("listenKey", listen_key, None).await;
        match response {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_exchange_info(&self) -> Result<ExchangeInfo, CustomError> {
        let response: Result<ExchangeInfo, CustomError> = self.get("exchangeInfo", None).await;
        match response {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    pub async fn get_available_coins_name(&self) -> Vec<String> {
        // if no exchange info then panic
        let exchange_info = self.get_exchange_info().await.unwrap();
        exchange_info
            .symbols
            .iter()
            .filter(|symbol| symbol.contractType == "PERPETUAL")
            .map(|symbol| symbol.symbol.clone())
            .collect()
    }
}
