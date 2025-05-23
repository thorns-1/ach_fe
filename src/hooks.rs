use crate::types::{Board, BoardInfo, FormInfo, Thread};
use gloo_net::http::Request;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[hook]
pub fn use_send_post_request(form_info: FormInfo) -> Callback<()> {
    use crate::config::use_config;
    let config = use_config();
    let url = format!("{}/post", config.base_url);

    let body = format!(
        "slug={}&name={}&options={}&subject={}&content={}&file={}",
        form_info.slug, form_info.name, form_info.options, form_info.subject, form_info.content, form_info.file
    );

    Callback::from(move |_| {
        let url = url.clone();
        let body = body.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let body = JsValue::from_str(&body);
            let _ = Request::post(&url)
                .body(body)
                .unwrap()
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
        });
    })
}

#[hook]
pub fn use_fetch<T: Clone + Default + 'static + serde::de::DeserializeOwned>(url: String) -> T {
    let data = use_state(|| T::default());

    {
        let data = data.clone();
        use_effect_with(url.clone(), move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: T = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                data.set(fetched_data);
            });
            || ()
        });
    }
    (*data).clone()
}

#[hook]
pub fn use_fetch_board(slug: &String) -> Board {
    use crate::config::use_config;
    let config = use_config();
    let url = format!("{}/board?slug={}", config.base_url, slug);
    use_fetch(url)
}

#[hook]
pub fn use_fetch_boards() -> Vec<BoardInfo> {
    use crate::config::use_config;
    let config = use_config();
    let url = format!("{}/boards", config.base_url);
    use_fetch(url)
}

#[hook]
pub fn use_fetch_thread(slug: &String, id: &String) -> Thread {
    use crate::config::use_config;
    let config = use_config();
    let url = format!("{}/board?slug={}&id={}", config.base_url, slug, id);
    use_fetch(url)
}
