use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::{Deserialize, Serialize};

mod schema;
use schema::niumms_room;

#[table_name = "niumms_room"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[derive(Debug)]
pub struct Room {
    pub id: Option<i32>,
    pub area_id: i32,
    pub room_name: String,
    pub desc: String,
    pub capacity: i32,
    pub status: i32,
}

impl Room {
    pub fn read(area_id: i32, connection: &MysqlConnection) -> Vec<Room> {
        niumms_room::table.filter(niumms_room::area_id.eq(&area_id)).load::<Room>(connection).unwrap()
    }
}
