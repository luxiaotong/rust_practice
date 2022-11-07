use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::{Deserialize, Serialize};

mod schema;
use schema::niumms_area;

#[table_name = "niumms_area"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
#[derive(Debug)]
pub struct Area {
    pub id: Option<i32>,
    pub area_name: String,
}

impl Area {
    pub fn read(connection: &MysqlConnection) -> Vec<Area> {
        niumms_area::table.order(niumms_area::id).load::<Area>(connection).unwrap()
    }
}
