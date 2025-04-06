use std::fs::File;

use mybops::{Error, List, RawList};
use mybops_web::{Item, RawItem};
use rusqlite::Connection;

fn main() {
    import().unwrap();
}

fn import() -> Result<(), Error> {
    let items: Vec<Item> = serde_json::from_reader(File::open("items.json").unwrap())?;
    let lists: Vec<List> = serde_json::from_reader(File::open("lists.json").unwrap())?;
    let mut conn = Connection::open("mybops")?;
    let tx = conn.transaction()?;
    for item in items {
        tx.execute(
            "INSERT INTO item (id, user_id, type, name, iframe, rating, user_score, user_wins, user_losses, metadata, hidden) VALUES (:id, :user_id, :type, :name, :iframe, :rating, :user_score, :user_wins, :user_losses, :metadata, :hidden)",
            serde_rusqlite::to_params_named(RawItem::from(item))?.to_slice().as_slice()
        )?;
    }
    for list in lists {
        tx.execute(
            "INSERT INTO list (id, user_id, mode, name, sources, iframe, items, favorite, query) VALUES (:id, :user_id, :mode, :name, :sources, :iframe, :items, :favorite, :query)",
            serde_rusqlite::to_params_named(RawList::from(list))?.to_slice().as_slice()
        )?;
    }
    tx.commit()?;
    Ok(())
}
