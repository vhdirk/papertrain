use esp_idf_svc::http::client::{Configuration, EspHttpConnection};

use embedded_svc::http::{client::Client as HttpClient, Method};
use log::*;
use querystring;

mod responses;
use responses::Connections;

#[derive(Debug, Clone)]
pub struct IRailConfig {
    pub url: &'static str,
    pub user_agent: &'static str,
}

pub struct IRailClient {
    config: IRailConfig,
    http: HttpClient<EspHttpConnection>,
}

fn create_client() -> anyhow::Result<HttpClient<EspHttpConnection>> {
    let config = Configuration {
        use_global_ca_store: true,
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    };

    Ok(HttpClient::wrap(EspHttpConnection::new(&config)?))
}


pub struct EspHttpConnectionBodyReader<'e>(&'e mut EspHttpConnection);

impl<'e> std::io::Read for EspHttpConnectionBodyReader<'e>{
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        self.0.read(buffer).map_err(|err| {
            std::io::Error::other(err)
        })
     }
}


impl IRailClient {

    pub fn new(config: IRailConfig) -> anyhow::Result<Self> {
        Ok(IRailClient {
            config,
            http: create_client()?
        })
    }


    fn get<T: serde::de::DeserializeOwned>(&mut self, path: &str, params: querystring::QueryParams) -> anyhow::Result<T> {
        let headers: [(&str, &str); 3] = [
            ("user-agent", self.config.user_agent),
            ("accept", "application/json"),
            ("connection", "close")
        ];

        let url = format!("{}{}?{}", self.config.url, path, querystring::stringify(params));

        let request = self.http.request(Method::Get, &url, &headers)?;
        info!("making request {}", url);
        let mut response = request.submit()?;

        let (_response_headers, body) = response.split();

        serde_json::from_reader(EspHttpConnectionBodyReader(body))
            .map_err(|err| anyhow::Error::new(err))

    }

    pub fn get_connections(&mut self, from: &str, to: &str) -> anyhow::Result<Connections> {
        let path = "/connections/";
        let params = vec![("from", from), ("to", to), ("format", "json")];


        self.get(path, params)
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