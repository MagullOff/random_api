use crate::types::{entry::ReturnEntry, entry::Entry, DBPool};
use crate::repository as EntryRepo;

pub fn insert_entry(url: String, conn: DBPool) -> Result<Entry,&'static str> {
    EntryRepo::insert_new(Entry::new(url), &conn)
}

pub fn get_all_entries(conn: DBPool) -> Result<Vec<ReturnEntry>, &'static str> {
    EntryRepo::get_all(&conn)
        .and_then(|vec| {
            Ok(vec
                .into_iter()
                .map(|e| ReturnEntry::new(e))
                .collect())
        })
}