use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("img.db").expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS favorites (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL,
                UNIQUE(url)
            );",
        ).unwrap();
        
        conn
    }
}

#[server]
pub async fn save_img(image: String) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("INSERT OR IGNORE INTO favorites (url) VALUES (?1)", &[&image]))?;
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
    DB.with(|f| f.execute("DELETE FROM favorites WHERE id = (?1)", &[&1]))?;
    Ok(())
}