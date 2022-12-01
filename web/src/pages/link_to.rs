use gloo_net::http::{ Request};
use yew::{function_component, Properties, html};
use yew_router::prelude::*;

use crate::router::Route;

use super::TinyData;

#[derive(Properties, PartialEq)]

pub struct RenderedAtProps{
    pub code: String,
}

#[function_component(LinkTo)]
pub fn link_to(prop: &RenderedAtProps) -> Html{
    let code = prop.code.clone();
    wasm_bindgen_futures::spawn_local(async move{
        let tiny_data_endpoint = format!("/api/s/{}", code);
        if let Ok(r) = Request::get(&tiny_data_endpoint).send().await{
            let ret: Result<TinyData<String>, _> = r.json().await;
            if let Ok(f) = ret{
                if let Err(_) = gloo_utils::window().location().set_href(&f.data){
                    log::info!("Something is wrong");
                }
            }
        }
    });
    html!{
        <>
            <Redirect<Route> to={Route::NotFound}/>
        </>
    }
}