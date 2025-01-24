use dioxus::prelude::*;
use rand::seq::SliceRandom;



#[component]
pub fn DogView() -> Element {

    // Change this to whatever album you want to fetch images from
    let album_identifier = "k4ad54";

    let mut img_src = use_resource(move || async move {
        let get_request = format!("https://eepy.ca/api/album/{album_identifier}/view");
        let response = reqwest::get(get_request)
            .await  
            .unwrap()
            .json::<EepyAPI>()
            .await
            .unwrap();

        let image = &response.album.files.choose(&mut rand::thread_rng()).unwrap();
        image.url.to_string()
    });

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() } 
        }
        div { id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "Skip" }
            button { id: "save",
                onclick: move |_| async move {
                    // Clone current image
                    let current = img_src.cloned().unwrap();
                    // Get a new image
                    img_src.restart();
                    // Save image 
                    crate::backend::save_img(current).await.unwrap();
                }, 
                "Save!"
            }
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