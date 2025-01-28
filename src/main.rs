use dioxus::prelude::*;

mod components;
use components::*;

mod backend;

static CSS: Asset = asset!("/assets/main.css");
static FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{href: CSS}
        document::Link { rel: "icon", href: FAVICON }
        
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,

    #[route("/add_albums")]
    Albums,
}
