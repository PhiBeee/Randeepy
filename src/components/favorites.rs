use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let favorites = use_resource(crate::backend::list_favorites).suspend()?;
    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (id, url) in favorites().unwrap() {
                    div {
                        key: id,
                        class: "favorite-dog",
                        img { src: "{url}"}
                    }
                }
            }
        }
    }
}