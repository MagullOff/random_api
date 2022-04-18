use std::time::SystemTime;
use crate::schema::entries;
use diesel::{self, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::prelude::*;

#[derive(Clone,Debug, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[table_name = "entries"]
pub struct Entry {
    pub id: Uuid,
    pub url: String,
    pub date: SystemTime,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.url==other.url && self.id == other.id 
    }
}

impl Entry {
    pub fn new(url: String) -> Entry {
        Entry{
            id: Uuid::new_v4(),
            url,
            date: SystemTime::now(),
        }
    }
}

#[derive(Clone,Debug, Deserialize, Serialize)]
pub struct ReturnEntry {
    pub url: String,
    pub date: String
}

impl ReturnEntry {
    pub fn new(entry: Entry) -> ReturnEntry {
        let datetime: DateTime<Utc> = entry.date.into();
        ReturnEntry {
            url: entry.url,
            date: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
        } 
    }
}