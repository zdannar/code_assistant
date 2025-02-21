pub mod http_server;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub model_name: String,
    #[serde(default = "http_serve_addr_default")]
    pub http_server_addr: String,
}

fn http_serve_addr_default() -> String {
    "0.0.0.0:8080".to_owned()
}
