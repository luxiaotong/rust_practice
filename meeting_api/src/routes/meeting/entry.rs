use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::{Deserialize, Serialize};
use diesel::debug_query;

mod schema;
use schema::niumms_entry;

#[table_name = "niumms_entry"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable, FromForm, Clone, Debug)]
pub struct Entry {
    pub id: Option<i32>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub room_id: Option<i32>,
    pub username: Option<String>,
    pub title: Option<String>,
    pub desc: Option<String>,
    pub status: Option<i32>,
}

impl Entry {
    pub fn create(entry: Entry, connection: &MysqlConnection) -> bool {
        diesel::insert_into(niumms_entry::table)
            .values(&entry)
            .execute(connection)
            .expect("Error creating new entry");

        true
    }

    pub fn check(entry: &Entry, connection: &MysqlConnection) -> i64 {
        niumms_entry::table.count()
            .filter(niumms_entry::room_id.eq(entry.room_id))
            .filter(niumms_entry::status.le(1))
            .filter(
                niumms_entry::start.le(&entry.start).and(niumms_entry::end.gt(&entry.start))
                .or(niumms_entry::start.lt(&entry.end).and(niumms_entry::end.ge(&entry.end)))
                .or(niumms_entry::start.ge(&entry.start).and(niumms_entry::end.le(&entry.end)))
            )
            .first::<i64>(connection).unwrap()
    }

    pub fn read(room_id: i32, date: String, connection: &MysqlConnection) -> Vec<Entry> {
        let mut start_time = String::from(&date);
        let mut end_time = String::from(&date);
        start_time.push_str(" 00:00:00");
        end_time.push_str(" 23:59:59");
        niumms_entry::table
            .filter(niumms_entry::room_id.eq(&room_id))
            .filter(niumms_entry::start.ge(&start_time))
            .filter(niumms_entry::start.le(&end_time))
            .order(niumms_entry::start_time.asc())
            .load::<Entry>(connection).unwrap()
    }

    pub fn find_by_id(id: i32, connection: &MysqlConnection) -> Entry {
        niumms_entry::table.find(id).first::<Entry>(connection).unwrap()
    }

    pub fn update(id: i32, status: i32, connection: &MysqlConnection) -> bool {
        diesel::update(niumms_entry::table.find(id))
            .set(niumms_entry::status.eq(&status))
            .execute(connection).is_ok()
    }

    pub fn test_debug() {
        let query = niumms_entry::table.count();
        let debug = debug_query::<diesel::mysql::Mysql, _>(&query);
        println!("query debug: {:?}", debug);
    }
}
