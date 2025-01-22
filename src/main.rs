use dioxus::{logger::tracing::info, prelude::*};
use rand::prelude::*;
use rand::seq::SliceRandom;
use serde::Deserialize;

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
    let mut new_img_src = use_signal(|| "".to_string());

    let album_identifier = "k4ad54";

    let fetch_new = move |_| async move {
        let _get_request = format!("https://eepy.ca/api/album/{album_identifier}/view");
        let response = reqwest::get("https://eepy.ca/api/album/k4ad54/view")
            .await  
            .unwrap()
            .json::<EepyAPI>()
            .await
            .unwrap();

        let image = &response.album.files.choose(&mut rand::thread_rng()).unwrap().url;
        new_img_src.set(image.to_string());
    };

    let save = move |_evt| {};

    rsx! {
        div { id: "dogview",
            img { src: "{new_img_src}" } 
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
    album: EepyAlbum
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EepyAlbum{
    name: String,
    description: Option<String>,
    is_nsfw: bool,
    count: i32,
    files: Vec<EepyFile>,
    cover: String
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct EepyFile {
    name: String,
    url: String,
    thumb: String,
    preview: String,
    uuid: String
}
