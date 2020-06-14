use async_trait::async_trait;
use rusty_pipe::downloader_trait::Downloader;
use rusty_pipe::youtube_extractor::error::ParsingError;
use std::collections::HashMap;
use std::future::Future;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yew::{Component, ComponentLink};
use wasm_bindgen::prelude::{*};
// use http::Response;

pub fn send_future<COMP: Component, F>(link: ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    spawn_local(async move {
        link.send_message(future.await);
    });
}

#[wasm_bindgen]
extern "C" {

    pub fn encodeURIComponent(uri:&str)->String;

    fn eval(code:&str)->String;

}


pub async fn fetch(url: &str, headers: HashMap<String, String>) -> Result<String, ParsingError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let urlencoded = encodeURIComponent(& base64::encode(url));
    let url = (&format!("https://rustypipe.deepraven.co/api/cors/{}", urlencoded));

    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|e| ParsingError::from(format!("{:#?}", e)))?;

    for header in headers {
        request
            .headers()
            .set(&header.0, &header.1)
            .map_err(|e| ParsingError::from(format!("{:#?}", e)))?;
    }

    let window: Window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| ParsingError::from(format!("{:#?}", e)))?;
    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(
        resp.text()
            .map_err(|e| ParsingError::from(format!("{:#?}", e)))?,
    )
    .await
    .map_err(|e| ParsingError::from(format!("{:#?}", e)))?;
    Ok(text.as_string().unwrap())
}

#[derive(Clone)]
pub struct DownloaderExample;

#[async_trait(?Send)]
impl Downloader for DownloaderExample {
    async fn download(url: &str) -> Result<String, ParsingError> {
        // println!("query url : {}", url);
        // let resp = reqwest::get(url)
        //     .await
        //     .map_err(|er| ParsingError::DownloadError {
        //         cause: er.to_string(),
        //     })?;
        // println!("got response ");
        // let body = resp
        //     .text()
        //     .await
        //     .map_err(|er| ParsingError::DownloadError {
        //         cause: er.to_string(),
        //     })?;
        // println!("suceess query");
        // Ok(String::from(body))
        fetch(url, HashMap::new()).await
        // Ok("".to_string())
    }

    async fn download_with_header(
        url: &str,
        header: HashMap<String, String>,
    ) -> Result<String, ParsingError> {
        // log::info!("downloadwith header {:#?}",header);
        // let urlencoded = base64::encode(url);
        // let url = format!("https://rustypipe.deepraven.co/api/cors/{}",urlencoded);
        // let client = reqwest::Client::new();
        // let res = client.get(&url);
        // let mut headers = reqwest::header::HeaderMap::new();
        // for header in header {
        //     headers.insert(
        //         reqwest::header::HeaderName::from_str(&header.0).map_err(|e| e.to_string())?,
        //         header.1.parse().unwrap(),
        //     );
        // }
        // // let res = res.headers(headers);
        // let res = res.send().await.map_err(|er| er.to_string())?;
        // let body = res.text().await.map_err(|er| er.to_string())?;
        // Ok(String::from(body))

        // Ok("".to_owned())
        fetch(url, header).await
    }

    fn eval_js(_script: &str) -> Result<String, String> {
        // println!("js result : {:?}", result);
        let result = eval(_script);
        print!("JS result: {}", result);
        Ok(result)
    }
}
