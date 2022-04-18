use diesel::{PgConnection, ConnectionError, Connection};
use lib::repository::{insert_new, get_all};
use lib::types::config::Config;
use lib::types::entry::Entry;
use diesel::result::Error;

pub fn establish_connection() -> Result<PgConnection, ConnectionError> {
    let database_url = Config::get_config().db_url;
    PgConnection::establish(&database_url)
}

#[test]
fn insert_entry_test() {
    let conn = establish_connection()
        .unwrap_or_else(|e| panic!("Error establishing conncection: '{}'",e));
    
    let test_new_entry = Entry::new("www.site.com".to_string());

    conn.test_transaction::<_,Error,_>(|| {
        insert_new(test_new_entry, &conn).unwrap();
        Ok(())
    })
}

#[test]
fn get_all_entries_test() {
    let conn = establish_connection()
        .unwrap_or_else(|e| panic!("Error establishing conncection: '{}'",e));
    
    let test_new_entry = Entry::new("www.site.com".to_string());

    conn.test_transaction::<_,Error,_>(|| {
        insert_new(test_new_entry.clone(), &conn).unwrap();
        let entry_vector = get_all(&conn).unwrap();
        if entry_vector.contains(&test_new_entry) == false {panic!();}
        Ok(())
    })
}
