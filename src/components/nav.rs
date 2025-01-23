use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div { id: "title",
            Link {to: Route::DogView,
                h1 {"Puppygirls and Dogwomen! ૮ ˶′ﻌ ‵˶ ა"}
            }
            Link { to: Route::Favorites, id: "heart", "💙"}
        }
        Outlet::<Route> {}
    }
}