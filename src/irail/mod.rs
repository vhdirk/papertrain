use core::{cmp::max, str::from_utf8};
use defmt::*;

use alloc::{borrow::ToOwned, string::String, vec::Vec};
use esp_backtrace as _;
use esp_println::println;

mod responses;
pub use responses::*;

use embedded_nal::{Dns, TcpConnect};
use reqwless::{
    client::{HttpClient, HttpRequestHandle, HttpResource, HttpResourceRequestBuilder},
    headers::ContentType,
    request::{DefaultRequestBuilder, Method, Request, RequestBuilder},
    response::Response,
};

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = [QueryParam<'a>];

/// Produces a URL query string from a given query by iterating through the vec.
///
/// # Examples
///
/// ```
/// extern crate querystring;
///
/// assert_eq!(querystring::stringify(vec![("foo", "bar"), ("baz", "qux")]), "?foo=bar&baz=qux&");
/// ```
pub fn stringify(query: &QueryParams) -> String {
    query.iter().fold(String::from("?"), |acc, &tuple| {
        acc + tuple.0 + "=" + tuple.1 + "&"
    })
}

pub struct IRailConfig {
    pub url: &'static str,
    pub user_agent: &'static str,
}

pub struct IRailClient<'a, 'c, T, D>
where
    T: TcpConnect + 'a,
    D: Dns + 'a,
{
    config: &'c IRailConfig,
    http: HttpClient<'a, T, D>,
}

impl<'a, 'c, T, D> IRailClient<'a, 'c, T, D>
where
    T: TcpConnect + 'a,
    D: Dns + 'a,
{
    pub fn new(config: &'c IRailConfig, client: HttpClient<'a, T, D>) -> Self {
        IRailClient {
            config,
            http: client,
        }
    }

    async fn get<'q, 'b, U>(
        &mut self,
        path: &str,
        params: &QueryParams<'q>,
        body: &'b mut [u8],
    ) -> anyhow::Result<U>
    where
        U: serde::de::Deserialize<'b>,
    {
        let headers: [(&str, &str); 3] = [
            ("user-agent", self.config.user_agent),
            ("accept", "application/json"),
            ("connection", "close"),
        ];

        let host_path = format!("{}{}", self.config.url, path);

        info!("Creating resource {}", host_path.as_str());

        let resource = self.http.resource(&host_path).await;
        info!("Resource creation done");

        if resource.is_err() {
            let err = resource.err().unwrap();
            warn!("Resource creation failed {}", err);
            anyhow::bail!("Resource creation failed")
        }

        info!("Unwrapping resource");

        let mut resource = resource.unwrap();

        let full_path = stringify(&params);
        info!("Building request: {}", full_path.as_str());

        let request = resource
            .get(full_path.as_str())
            .headers(&headers)
            .content_type(ContentType::ApplicationJson);

        info!("Sending request");

        let mut header_buf = [0; 1024];
        let response = request.send(&mut header_buf).await;

        info!("Got response");

        if response.is_err() {
            let err = response.unwrap_err();
            warn!("Response error {}", err);
            anyhow::bail!("error")
        }

        let response = response.unwrap();
        let status = response.status;
        info!("Reponse status {}", status);

        let content_length = response.content_length.unwrap_or(0);
        let resonse_body = response.body();

        info!("Reponse size {}", content_length);
        let mut body_reader = resonse_body.reader();

        info!("Reading response");

        let read_result = body_reader.read_to_end(body).await;
        info!("Done reading {}", read_result.is_ok());

        if read_result.is_err() {
            let err = read_result.unwrap_err();
            warn!("error {}", err);
            anyhow::bail!("error")
        }

        let num_bytes = read_result.unwrap();
        info!("Read {} bytes", num_bytes);

        let result = serde_json::from_slice(&body[..num_bytes]);

        if result.is_err() {
            let error = result.err().unwrap();
            let err_str = format!("{:?}", error);
            warn!("error {:?}", err_str.as_str());
            anyhow::bail!("result")
        }

        Ok(result.unwrap())
    }

    pub async fn get_connections(&mut self, from: &str, to: &str) -> anyhow::Result<Connections> {
        let path = "/connections";
        let params = [
            ("from", from),
            ("to", to),
            ("format", "json"),
            ("results", "1"),
        ];
        let mut buffer: Vec<u8> = vec![0; 30 * 1024];

        let result = self.get(path, &params, buffer.as_mut_slice()).await;
        drop(buffer);
        if result.is_err() {
            anyhow::bail!("result")
        }
        Ok(result.unwrap())
    }
}
