use crate::schema::entries::table as all_entries;
use crate::types::entry::Entry;
use diesel::{self, PgConnection};
use diesel::prelude::*; 

pub fn insert_new(new_entry: Entry, conn: &PgConnection) -> Result<Entry, &'static str> {
    diesel::insert_into(all_entries)
        .values(&new_entry)
        .get_result::<Entry>(conn)
        .map_err(|_| "Insert failed")
}

pub fn get_all(conn: &PgConnection) -> Result<Vec<Entry>,&'static str> {
    all_entries.load::<Entry>(conn)
        .map_err(|_| "Error getting entries")
}
