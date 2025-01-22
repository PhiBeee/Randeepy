use dioxus::{logger::tracing::info, prelude::*};
use rand::seq::SliceRandom;

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet{href: CSS}
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    rsx!(
        div { id: "title",
            h1 { "Puppygirls and Dogwomen! ૮ ˶′ﻌ ‵˶ ა" }
        }
    )
}

#[component]
fn DogView() -> Element {
    let img_src = use_hook(|| "https://eepy.ca/silly-FB9xRuikhqoP.gif");

    let mut new_img_src = use_signal(|| "".to_string());

    let album_identifier = "k4ad54";

    let fetch_new = move |_| async move {
        let get_request = format!("https://eepy.ca/api/album/{album_identifier}/view");
        let response = reqwest::get(get_request)
            .await  
            .unwrap()
            .json::<EepyAPI>()
            .await
            .unwrap();

        new_img_src.set(response.files.choose(&mut rand::thread_rng()).unwrap().to_string());
    };

    let save = move |evt| {};

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" } 
        }
        div { id: "buttons",
            button { onclick: fetch_new, id: "skip", "Skip" }
            button { onclick: save, id: "save", "Save!" }
        }
    }
}

#[derive(serde::Deserialize)]
struct EepyAPI {
    message: String,
    files: Vec<String>
}
