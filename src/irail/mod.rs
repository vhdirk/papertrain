
use defmt::*;
use core::str::from_utf8;

use alloc::{string::String, borrow::ToOwned};
use esp_backtrace as _;
use esp_println::println;

// use esp_idf_svc::http::client::{Configuration, EspHttpConnection};

// use embedded_svc::http::{client::Client as HttpClient, Method};
// use log::*;
// use querystring;

mod responses;
use responses::Connections;

use reqwless::{client::{HttpClient, HttpResourceRequestBuilder, HttpRequestHandle, HttpResource}, headers::ContentType, request::{Request, Method, RequestBuilder, DefaultRequestBuilder}, response::Response};
use embedded_nal::{TcpConnect, Dns};

pub type QueryParam<'a> = (&'a str, &'a str);
pub type QueryParams<'a> = [QueryParam<'a>];

/// Produces a URL query string from a given query by iterating through the vec.
///
/// # Examples
///
/// ```
/// extern crate querystring;
///
/// assert_eq!(querystring::stringify(vec![("foo", "bar"), ("baz", "qux")]), "foo=bar&baz=qux&");
/// ```
pub fn stringify(query: &QueryParams) -> String {
    query.iter().fold(String::new(), |acc, &tuple| {
        acc + tuple.0 + "=" + tuple.1 + "&"
    })
}

// [derive(Debug, Clone)]
pub struct IRailConfig {
    pub url: &'static str,
    pub user_agent: &'static str,
}

pub struct IRailClient<'a, 'c, T, D>
where
    T: TcpConnect + 'a,
    D: Dns + 'a
{
    config: &'c IRailConfig,
    http: HttpClient<'a, T, D>,
}

// fn create_client() -> anyhow::Result<HttpClient<EspHttpConnection>> {
//     let config = Configuration {
//         use_global_ca_store: true,
//         crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
//         ..Default::default()
//     };

//     Ok(HttpClient::wrap(EspHttpConnection::new(&config)?))
// }

// pub struct EspHttpConnectionBodyReader<'e>(&'e mut EspHttpConnection);

// impl<'e> std::io::Read for EspHttpConnectionBodyReader<'e> {
//     fn read(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
//         self.0
//             .read(buffer)
//             .map_err(|err| std::io::Error::other(err))
//     }
// }

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

    // async fn get<'q, U>(
    //     &mut self,
    //     path: &str,
    //     params: QueryParams<'q>,
    // ) -> anyhow::Result<T>
    // where U: serde::de::DeserializeOwned {
    //     let headers: [(&str, &str); 3] = [
    //         ("user-agent", self.config.user_agent),
    //         ("accept", "application/json"),
    //         ("connection", "close"),
    //     ];

    //     let url = format!(
    //         "{}{}?{}",
    //         self.config.url,
    //         path,
    //         stringify(params)
    //     );

    //     let builder = match self.http.request(Method::GET, &self.config.url).await {
    //         Ok(builder) => builder,
    //         Err(err) => {
    //             anyhow::bail!("{:?}", err);
    //         }
    //     };

    //     builder. .headers(headers);

    //     info!("making request {}", url);
    //     let mut response = request.submit()?;

    //     let (_response_headers, body) = response.split();

    //     // let response = serde_json_core::from_reader(EspHttpConnectionBodyReader(body))
    //     //     .map_err(|err| anyhow::Error::new(err));

    //     info!("got response {:?}", response);

    //     response
    // }

    pub async fn get_connections(&mut self, from: &str, to: &str) -> anyhow::Result<()> {
        let headers: [(&str, &str); 3] = [
            ("user-agent", self.config.user_agent),
            ("accept", "application/json"),
            ("connection", "close"),
        ];
        let path = "/connections/";
        let params = [("from", from), ("to", to), ("format", "json")];

        // let builder = HttpResourceRequestBuilder {
        //     conn: &mut self.http,
        //     request: Request::new(Method::GET, path).host(self.config.url),
        //     base_path: self.base_path,
        // };
        info!("Creating resource {}", "lalala");

        let mut host_path: String = self.config.url.into();
        host_path.push_str(path);

        // info!("Creating resource {}", &host_path);
        let resource = self.http.resource("https://api.irail.be/connections/");
        info!("Resource creation done");

        let resource = resource.await;
        info!("Resource creation done");

        if resource.is_err() {
            info!("Resource creation failed");
            let err = resource.err().unwrap();
            anyhow::bail!("Resource creation failed: {:?}", err)
        }

        info!("Unwrapping resource");

        let mut resource = resource.unwrap();

        let full_path = format!("{}{}", path, stringify(&params));
        info!("Building request: {}", full_path.as_str());
        let request = Request::get(full_path.as_str())
        .headers(&headers)
        .content_type(ContentType::ApplicationJson)
        .build();

        info!("Sending request");

        let mut rx_buf = [0; 128*1024];
        let response = resource.send(request, &mut rx_buf).await;

        info!("Got response");

        if response.is_err() {
            anyhow::bail!("error")
        }
        Ok(())

        // let body = from_utf8(response.unwrap().body().read_to_end().await.unwrap()).unwrap();


        // let result = serde_json_core::from_str(&body);
        // if result.is_err() {
        //     anyhow::bail!("result")
        // }

        // let result: (Connections, usize) = result.unwrap();

        // // let request = handle.unwrap()
        // //     .host(self.config.url)
        // //     .path(format!("{}{}", path, stringify(&params)).as_str())
        // //     .headers(&headers)
        // //     .content_type(ContentType::ApplicationJson)
        // //     .build();

        // // request.(self.http);

        // // request = request.headers(&headers).content_type(ContentType::ApplicationJson).
        // Ok(result.0)
        // self.get(path, params)
    }
}

// pub fn request_image(image_data_url: &str) -> anyhow::Result<Vec<u8>> {
//     let mut client = create_client()?;

//     get_data(&mut client, image_data_url)
// }

// fn create_client() -> anyhow::Result<HttpClient<EspHttpConnection>> {
//     let config = Configuration {
//         use_global_ca_store: true,
//         crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
//         ..Default::default()
//     };

//     Ok(HttpClient::wrap(EspHttpConnection::new(&config)?))
// }

// // This code is mostly taken from some sample code somewhere
// fn get_data(client: &mut HttpClient<EspHttpConnection>, url: &str) -> anyhow::Result<Vec<u8>> {
//     let headers: [(&str, &str); 2] = [("accept", "application/octet-stream"), ("connection", "close")];
//     let request = client.request(Method::Get, &url, &headers)?;
//     info!("making request {}", url);
//     let mut response = request.submit()?;

//     // Process response
//     let status = response.status();
//     info!("response status: {}", status);
//     if status != 200 {
//         anyhow::bail!("response status was not 200: {}", status);
//     }

//     let buffer_size = crate::get_buffer_size();
//     let (_headers, mut body) = response.split();
//     let mut buf = vec![0u8; buffer_size];
//     let bytes_read = io::try_read_full(&mut body, &mut buf).map_err(|e| e.0)?;
//     info!("Read {} bytes", bytes_read);

//     // Drain the remaining response bytes
//     // TODO: probably should error here since we should get exactly the buffer size
//     while body.read(&mut buf)? > 0 {}

//     if bytes_read == 0 {
//         anyhow::bail!("Image data body was empty");
//     }
//     if bytes_read != buffer_size {
//         anyhow::bail!("Image data body was wrong size. Expected {}, got {}", buffer_size, bytes_read);
//     }
//     Ok(buf)
// }

// }
