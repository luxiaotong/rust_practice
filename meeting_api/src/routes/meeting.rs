use super::super::db;
use serde_json::Value;
use serde_json::json;
use rocket_contrib::json::Json;
use chrono::NaiveDateTime;
use chrono::NaiveDate;
use chrono_tz::Asia::Shanghai;
use chrono::TimeZone;

mod area;
use area::Area;
mod room;
use room::Room;
mod area_room;
use area_room::AreaRoom;
mod entry;
use entry::Entry;

#[get("/meeting/area_list")]
pub fn area_list(connection: db::Connection) -> Json<Value> {
    Json(json!(Area::read(&connection)))
}

#[get("/meeting/room_list?<area_id>")]
pub fn room_list(area_id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!(Room::read(area_id, &connection)))
}

#[get("/meeting/room_detail?<id>")]
pub fn room_detail(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!(AreaRoom::get_by_id(id, &connection)))
}

#[post("/meeting/create", format="application/json", data="<entryreq>")]
pub fn create(entryreq: Json<Entry>, connection: db::Connection) -> Json<Value> {
    println!("entryreq: {:?}", entryreq);
    let get_timestamp = |datetime_str| {
        let dt = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").unwrap();
        Shanghai.from_local_datetime(&dt).unwrap().timestamp()
    };

    let entry = Entry {
        start_time: Some(get_timestamp(entryreq.start.as_ref().unwrap())),
        end_time: Some(get_timestamp(entryreq.end.as_ref().unwrap())),
        ..entryreq.into_inner()
    };

    let count:i64 = Entry::check(&entry, &connection);
    if count == 0 {
        Entry::create(entry, &connection);
        return Json(json!({"status": 200, "message": "预订成功"}));
    } else {
        return Json(json!({"status": 0, "message": "已被预订"}));
    }
}

#[get("/meeting/entry_list?<room_id>&<date>")]
pub fn entry_list(room_id: i32, date:String, connection: db::Connection) -> Json<Value> {
    let curr_date = String::from(&date);
    let entry_list: Vec<Entry> = Entry::read(room_id, date, &connection);
    let d = NaiveDate::parse_from_str(&curr_date, "%Y-%m-%d").unwrap();
    let mut entry_result: Vec<Entry> = Vec::new();
    for i in 8..19 {
        let ndt_s = d.and_hms(i, 0, 0);
        let ndt_e = d.and_hms(i+1, 0, 0);
        //println!("ndt_s: {:?}, ndt_e: {:?}", ndt_s, ndt_e);
        let ts_s = Shanghai.from_local_datetime(&ndt_s).unwrap().timestamp();
        let ts_e = Shanghai.from_local_datetime(&ndt_e).unwrap().timestamp();
        //println!("ts_s: {:?}, ts_e: {:?}", ts_s, ts_e);
        let dt_s = Shanghai.from_local_datetime(&ndt_s).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        let dt_e = Shanghai.from_local_datetime(&ndt_e).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
        //println!("dt_s: {:?}, dt_e: {:?}", dt_s, dt_e);

        match entry_list.iter().position(|e| e.start_time.unwrap() <= ts_s && e.end_time.unwrap() >= ts_e) {
            Some(pos) => {
                let mut e = entry_list[pos].clone();
                e.start_time = Some(ts_s);
                e.end_time = Some(ts_e);
                e.start = Some(dt_s);
                e.end = Some(dt_e);
                entry_result.push(e);
            },
            None => {
                let e = Entry {
                    id: Some(0),
                    start_time: Some(ts_s),
                    end_time: Some(ts_e),
                    start: Some(dt_s),
                    end: Some(dt_e),
                    room_id: Some(0),
                    username: Some(String::from("")),
                    title: Some(String::from("")),
                    desc: Some(String::from("")),
                    status: Some(0),
                };
                entry_result.push(e);
            },
        };
    }

    Json(json!(entry_result))
}

#[get("/meeting/entry_detail?<id>")]
pub fn entry_detail(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!(Entry::find_by_id(id, &connection)))
}

#[post("/meeting/cancel", format="application/json", data="<entryreq>")]
pub fn cancel(entryreq: Json<Entry>, connection: db::Connection) -> Json<Value> {
    Entry::update(entryreq.id.unwrap(), 3, &connection);
    Json(json!({"status": 200, "message": "取消预订成功"}))
}

#[post("/meeting/checkin", format="application/json", data="<entryreq>")]
pub fn checkin(entryreq: Json<Entry>, connection: db::Connection) -> Json<Value> {
    println!("params: {:?}", entryreq);
    //Entry::update(entryreq.id.unwrap(), 2, &connection);
    Json(json!({"status": 200, "message": "签到成功"}))
}
