use reqwest;

pub enum FetchURLType {
    WASM,
    JS,
}

impl FetchURLType {
    fn url(&self) -> &str {
        match self {
            FetchURLType::WASM => "https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/180px-WebAssembly_Logo.svg.png",
            FetchURLType::JS => "https://upload.wikimedia.org/wikipedia/commons/thumb/9/99/Unofficial_JavaScript_logo_2.svg/480px-Unofficial_JavaScript_logo_2.svg.png"
        }
    }
}

pub async fn fetch_image(
    url_type: FetchURLType,
) -> std::result::Result<reqwest::Response, reqwest::Error> {
    reqwest::Client::new().get(url_type.url()).send().await
}
