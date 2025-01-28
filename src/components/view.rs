use dioxus::prelude::*;
use rand::seq::SliceRandom;

static ALBUM: GlobalSignal<String> = Global::new(|| "k4ad54".to_string());

#[component]
pub fn DogView() -> Element {

    let mut albums = use_signal(|| vec![("Dog Posting".to_string(),"k4ad54".to_string())]);

    let get_album_list = move |_| async move {
        let buh = crate::backend::list_albums()
            .await 
            .unwrap();

        albums.set(buh);
    };



    let mut img_src = use_resource(move || async move {
        let get_request = format!("https://eepy.ca/api/album/{ALBUM}/view");
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
            // button { onclick: get_album_list, "Get Albums!"}
            select {  
                onmounted: get_album_list,
                onchange: move |evt| {
                    *ALBUM.write() = evt.value();
                    img_src.restart();
                },
                for (name, url) in albums.cloned() {
                    option {  
                        value: url,
                        label: name,
                        onchange: move |_| {}
                    }
                }
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct EepyAPI {
    album: EepyAlbum
}

#[derive(serde::Deserialize)]
struct EepyAlbum{
    files: Vec<EepyFile>,
}

#[derive(serde::Deserialize)]
struct EepyFile {
    url: String,
}