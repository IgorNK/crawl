use crate::todos::Todo;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::sync::{mpsc::Sender, Arc};
use thiserror::Error;

// #[cfg(not(target_arch = "wasm32"))]
use reqwest::Method;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_arch = "wasm32")]
use web_sys::{Request, RequestInit, RequestMode, Response};
// use reqwest_wasm::Method;

const URL: &str = "https://simple-api.metsysfhtagn.repl.co/api/todos";

#[derive(Error, Debug)]
pub enum ApiError {
    // #[cfg(not(target_arch = "wasm32"))]
    #[error("Unable to send request")]
    SendRequestError(#[from] reqwest::Error),
    // #[cfg(target_arch = "wasm32")]
    // #[error("Unable to send web request")]
    // WebRequestError(#[from] reqwest::Error),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
}

pub enum ResponseData {
    GetResponse(Result<Vec<Todo>, ApiError>),
    PostResponse(Result<Todo, ApiError>),
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct ResponseTodos {
    status: String,
    results: u32,
    todos: Vec<Todo>,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct ResponsePost {
    status: String,
    data: TodoData,
}

#[derive(Deserialize, Serialize, Default, Debug)]
struct TodoData {
    todo: Todo,
}

// Native

#[cfg(not(target_arch = "wasm32"))]
pub fn get_todos(tx: Sender<ResponseData>) {
    dbg!("Get todos call");
    tokio::spawn(async move {
        let body: String = reqwest::get(URL)
            .await
            .expect("Failed to fetch data from server")
            .text()
            .await
            .expect("Failed to parse data to text");

        let result: ResponseTodos = serde_json::from_str(&body).unwrap_or(ResponseTodos::default());
        dbg!(&result);
        let _ = tx.send(ResponseData::GetResponse(Ok(result.todos)));
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn create_todo(todo: Todo, tx: Sender<ResponseData>) {
    dbg!("Create todo call");
    tokio::spawn(async move {
        let response = post_todo(todo).await;
        let _ = tx.send(ResponseData::PostResponse(response));
    });
}

#[cfg(not(target_arch = "wasm32"))]
async fn post_todo(todo: Todo) -> Result<Todo, ApiError> {
    let client = reqwest::Client::new();
    let request = client
        .request(Method::POST, URL)
        .json(&todo)
        .build()
        .map_err(ApiError::SendRequestError)?;

    let response: ResponsePost = client
        .execute(request)
        .await?
        .json()
        .await
        .map_err(ApiError::SendRequestError)?;

    dbg!(&response);
    match response.status.as_str() {
        "success" => Ok(response.data.todo),
        _ => Err(ApiError::BadRequest("Unknown error")),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_image(url: String, sender: Sender<Arc<Bytes>>) {
    tokio::spawn(async move {
        let static_url = "https://images.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png";
        let url_parsed = reqwest::Url::parse(static_url).unwrap();
        log::warn!("{}", &url_parsed);
        let client = reqwest::Client::new();
        let request = client
            .request(Method::GET, url_parsed)
            .header("Content-Type", "application/json")
            .header(reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .build()
            .unwrap();

        log::warn!("Pre-request");

        let response = client.execute(request).await;

        log::warn!("We got a response");

        let bytes = response
            .unwrap()
            .bytes()
            .await
            .expect("Failed to parse byte image data");

        let result = Arc::new(bytes);
        let _ = sender.send(result);
    });
}

// WebAssembly

#[cfg(target_arch = "wasm32")]
pub fn get_todos_web(tx: Sender<ResponseData>) {
    wasm_bindgen_futures::spawn_local(async move {
        let body: String = reqwest::get(URL)
            .await
            .expect("Failed to fetch data from server")
            .text()
            .await
            .expect("Failed to parse data to text");

        let result: ResponseTodos = serde_json::from_str(&body).unwrap_or(ResponseTodos::default());
        let _ = tx.send(ResponseData::GetResponse(Ok(result.todos)));
    });
}

#[cfg(target_arch = "wasm32")]
pub fn create_todo_web(todo: Todo, tx: Sender<ResponseData>) {
    wasm_bindgen_futures::spawn_local(async move {
        let response = post_todo_web(todo).await;
        let _ = tx.send(ResponseData::PostResponse(response));
    });
}

#[cfg(target_arch = "wasm32")]
async fn post_todo_web(todo: Todo) -> Result<Todo, ApiError> {
    let client = reqwest::Client::new();
    let request = client
        .request(Method::POST, URL)
        .json(&todo)
        .build()
        .map_err(ApiError::SendRequestError)?;

    let response: ResponsePost = client
        .execute(request)
        .await?
        .json()
        .await
        .map_err(ApiError::SendRequestError)?;

    match response.status.as_str() {
        "success" => Ok(response.data.todo),
        _ => Err(ApiError::BadRequest("Unknown error")),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn fetch_image_web(url: String, sender: Sender<Arc<Bytes>>) {
    // log::warn!("{}", url);
    wasm_bindgen_futures::spawn_local(async move {
        let static_url = "https://images.google.com/images/branding/googlelogo/1x/googlelogo_light_color_272x92dp.png";
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::NoCors);
        let request = Request::new_with_str_and_init(&static_url), &opts).expect("Failed to form request");
        request
            .headers()
            .set("Accept", "image/png")
            .expect("Failed to set request headers");

        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.expect("Failed to send request");
        let response: Response = resp_value.dyn_into().unwrap();

        log::warn!("We got a response");
  
        let bytes: Bytes = response.dyn_into().unwrap();        

        let result = Arc::new(bytes);
        let _ = sender.send(result);
    });
}
