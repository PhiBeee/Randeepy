use dioxus::prelude::*;

#[component]
pub fn Albums() -> Element{
    let mut albums = use_resource(crate::backend::list_albums);
    let albums_signal = albums.suspend()?;

    let mut new_album_name = use_signal(|| "".to_string());
    let mut new_album_url = use_signal(|| "".to_string());

    let mut selected_album = use_signal(|| "".to_string());

    let add_album = move |_| async move {
        let url_valid = check_url(new_album_url).await;
        if url_valid{
            crate::backend::add_albums(new_album_name.to_string(), new_album_url.to_string()).await.unwrap();
            albums.restart()
        }
    };


    rsx! {
        div { id : "dogview",  
            div { id: "labels",
                input {  
                    placeholder: "Album name",
                    oninput: move |evt| new_album_name.set(evt.value())
                }
                input {  
                    placeholder: "Public url end",
                    maxlength: 6, // Chibisafe public urls end with a 6 char long identifier
                    oninput: move |evt| new_album_url.set(evt.value())
                }
            }
            div {  id: "buttons",
                button { 
                    onclick: add_album,
                    "Add album"
                }
                select {  
                    onchange: move |evt| {
                        selected_album.set(evt.value());
                    },
                    for (name, url) in albums_signal().unwrap() {
                        option {  
                            value: url,
                            label: name,
                            onchange: move |_| {}
                        }
                    }
                }
                button {  
                    onclick: move |_| async move {
                        crate::backend::rm_albums(selected_album.to_string()).await.unwrap();
                        albums.restart();
                    },
                    "Remove album"
                }
                
            }
        }
    }
}

async fn check_url(url: Signal<String>) -> bool {
    let get_request = format!("https://eepy.ca/api/album/{url}/view");
    let response_code = reqwest::get(get_request)
        .await
        .unwrap()
        .status()
        .as_u16();

    match response_code {
        200 => true,
        _ => false,
    }
}