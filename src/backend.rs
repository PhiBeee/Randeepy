use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("img.db").expect("Failed to open database");

        conn.execute_batch(
            "
            BEGIN;
            CREATE TABLE IF NOT EXISTS favorites (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL,
                UNIQUE(url)
            );
            CREATE TABLE IF NOT EXISTS albums (
                name TEXT PRIMARY KEY,
                url TEXT NOT NULL,
                UNIQUE(url)
            );
            INSERT OR IGNORE INTO albums (name, url) VALUES('Doggirls', 'k4ad54');
            COMMIT;",
        ).unwrap();
        
        conn
    }
}

#[server]
pub async fn list_albums()-> Result<Vec<(String, String)>, ServerFnError> {
    let albums = DB.with(|f| {
        f.prepare("SELECT name, url FROM albums ORDER BY name DESC")
        .unwrap()
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap()
        .map(|r| r.unwrap())
        .collect()
    });

    Ok(albums)
}

#[server]
pub async fn get_album(name: String) -> Result<String, ServerFnError> {
    let album_url = DB.with(|f| f.execute("SELECT url FROM albums WHERE name = (?1)", &[&name]))?;
    Ok(album_url.to_string())
}

#[server]
pub async fn save_img(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT OR IGNORE INTO favorites (url) VALUES (?1)", &[&image]))?;
    Ok(())
}

#[server]
pub async fn add_albums(name: String, url: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT OR IGNORE INTO albums VALUES (?1, ?2)", rusqlite::params![name, url]))?;
    Ok(())
}

#[server]
pub async fn rm_albums(url: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("DELETE FROM albums WHERE url = ?1", rusqlite::params![url]))?;
    Ok(())
}

#[server]
pub async fn list_favorites() -> Result<Vec<(usize, String)>, ServerFnError> {
    let favorites = DB.with(|f| {
        f.prepare("SELECT id, url FROM favorites ORDER BY id DESC")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(favorites)
}

#[server]
pub async fn remove_favorite(id: usize) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("DELETE FROM favorites WHERE id = (?1)", &[&id]))?;
    Ok(())
}

#[server]
pub async fn remove_all() -> Result<(), ServerFnError> {
    DB.with(|f| f.execute_batch("DELETE FROM favorites"))?;
    Ok(())
}