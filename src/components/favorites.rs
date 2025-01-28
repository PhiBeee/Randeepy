use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let mut favorites = use_resource(crate::backend::list_favorites);
    let favorites_signal = favorites.suspend()?;
    rsx! {
        div { id: "favorites",
            div { id: "buttons",
                button {  id: "save",
                    onclick: move |_| async move {
                        crate::backend::remove_all().await.unwrap();
                        favorites.restart();
                    },
                    "DELETE ALL FAVORITES"
                }
            }
            div { id: "favorites-container",
                for (id, url) in favorites_signal().unwrap(){
                    div {
                        key: id,
                        class: "favorite-dog",
                        img { src: "{url}"},
                        button { onclick: move |_| async move {
                            crate::backend::remove_favorite(id).await.unwrap();
                            favorites.restart();
                            },
                        "🗑️"
                        }
                        a {
                            class: "favorite-dog",
                            href: "{url}",
                            download: "",
                            "⬇️" 
                        }
                    }
                }
            }
        }
    }
}