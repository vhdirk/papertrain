use defmt::*;

use alloc::{string::String, vec::Vec, string::ToString};
use embedded_nal_async::{Dns, SocketAddr, TcpConnect};
use esp_backtrace as _;

mod responses;
pub use responses::*;

use reqwless::{client::HttpClient, headers::ContentType, request::RequestBuilder};

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = [QueryParam<'a>];

pub fn stringify(query: &QueryParams) -> String {
    query.iter().fold(String::from("?"), |acc, &tuple| {
        acc + tuple.0 + "=" + tuple.1 + "&"
    })
}

#[derive(Debug, Format)]
pub enum IRailError {
    NetworkError(reqwless::Error),
    ParseError(#[defmt(Debug2Format)]serde_json::Error),
}

impl From<reqwless::Error> for IRailError {
    fn from(value: reqwless::Error) -> Self {
        IRailError::NetworkError(value)
    }
}

impl From<serde_json::Error> for IRailError {
    fn from(value: serde_json::Error) -> Self {
        IRailError::ParseError(value)
    }
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
    ) -> Result<U, IRailError>
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

        let mut resource = self.http.resource(&host_path).await?;
        info!("Resource creation done");

        let full_path = stringify(&params);
        info!("Building request: {}", full_path.as_str());

        let request = resource
            .get(full_path.as_str())
            .headers(&headers)
            .content_type(ContentType::ApplicationJson);

        info!("Sending request");

        let mut header_buf = vec![0; 1024];
        let response = request.send(&mut header_buf).await?;

        info!("Got response");

        let status = response.status;
        info!("Reponse status {}", status);

        let content_length = response.content_length.unwrap_or(0);
        let resonse_body = response.body();

        info!("Reponse size {}", content_length);
        let mut body_reader = resonse_body.reader();

        info!("Reading response");

        let num_bytes = body_reader.read_to_end(body).await?;

        info!("Read {} bytes", num_bytes);

        Ok(serde_json::from_slice::<U>(&body[..num_bytes])?)
    }

    pub async fn get_connections(
        &mut self,
        from: &str,
        to: &str,
        results: Option<u8>
    ) -> Result<Connections, IRailError> {
        let num_results = results.unwrap_or(1);
        let path = "/connections";
        let params = [
            ("from", from),
            ("to", to),
            ("format", "json"),
            ("results", &num_results.to_string()),
        ];
        let mut buffer: Vec<u8> = vec![0; 50 * 1024];

        let mut result: Connections = self.get(path, &params, &mut buffer).await?;

        drop(buffer);
        // save space by removing stuff we certainly won't need
        result.connections.truncate(num_results as usize);

        Ok(result)
    }
}
