table! {
    niumms_area {
        id -> Nullable<Integer>,
        area_name -> Varchar,
    }
}

table! {
    niumms_room {
        id -> Nullable<Integer>,
        area_id -> Nullable<Integer>,
        room_name -> Varchar,
        desc -> Varchar,
        capacity -> Integer,
        status -> Integer,
    }
}
    
joinable!(niumms_room -> niumms_area (area_id));

allow_tables_to_appear_in_same_query!(
    niumms_room,
    niumms_area,
);

