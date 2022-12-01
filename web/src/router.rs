use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::Create;
use crate::pages::LinkTo;
use crate::pages::Links;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/links")]
    Links,
    #[at("/create")]
    Create,
    #[at("/404")]
    NotFound,
    #[at("/:code")]
    LinkTo { code: String },
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! {
            <h1 class="text-3xl font-bold underline">
                {"Hello world!"}
            </h1>
        },
        Route::Links => html! {<Links/>},
        Route::Create => html! {<Create/>},
        Route::LinkTo { code } => html! {<LinkTo code = {code.clone()} />},
        Route::NotFound => html! {<h1>{"404"}</h1>},
    }
}
