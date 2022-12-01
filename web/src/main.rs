use router::{Route, switch};
use yew::prelude::*;
use yew_router::prelude::*;

mod router;
mod pages;

#[function_component(App)]
pub fn app() -> Html{
    html!{
       <BrowserRouter>
        <Switch<Route> render={Switch::render(switch)}/>
       </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}
