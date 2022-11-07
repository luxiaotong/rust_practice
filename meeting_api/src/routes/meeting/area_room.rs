use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::{Deserialize, Serialize};
use diesel::debug_query;

mod schema;
use schema::{niumms_room, niumms_area};

//#[table_name = "niumms_room"]
//#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[derive(Serialize, Deserialize, Queryable)]
#[derive(Debug)]
pub struct AreaRoom {
    pub id: Option<i32>,
    pub room_name: String,
    pub desc: String,
    pub capacity: i32,
    pub area_name: String,
}

impl AreaRoom {
    pub fn get_by_id(id: i32, connection: &MysqlConnection) -> AreaRoom {
        niumms_room::table.find(id)
            .inner_join(niumms_area::table.on(niumms_area::id.eq(niumms_room::area_id)))
            .select((niumms_room::id, niumms_room::room_name, niumms_room::desc, niumms_room::capacity, niumms_area::area_name))
            .first::<AreaRoom>(connection).unwrap()
    }
}

