use rand::{Rng, thread_rng};
use curl::easy::Easy;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use chrono::Utc;
use std::fs;
use rocket::State;
use crate::EnvConf;
use rocket_contrib::json::Json;


#[derive(Serialize)]
pub struct ReturnData {
    status: u8,
    data: SignPackage,
}

#[derive(Serialize, Deserialize)]
struct SignPackage {
    appid: String,
    nonce_str: String,
    timestamp: String,
    url: String,
    signature: String,
    raw_string: String,
}

#[derive(Serialize, Deserialize)]
struct JsApiTicketCache {
    ticket: String,
    expire_time: i64,
}

#[derive(Serialize, Deserialize)]
struct AccessTokenCache {
    access_token: String,
    expire_time: i64,
}


#[get("/wechat/sign?<url>")]
pub fn sign(url: String, env_conf: State<EnvConf>) -> Json<ReturnData> {
    let sign_package = get_sign_package(
        url,
        env_conf.appid.to_string(),
        env_conf.secret.to_string()
    );
    //let res_json = serde_json::to_string(&sign_package).unwrap();
    //res_json.to_string()
    Json(ReturnData{status:200, data:sign_package})
}


fn get_sign_package(url:String, appid:String, secret:String) -> SignPackage {
    println!("url: {}", url);
    println!("appid: {}", appid);
    println!("secret: {}", secret);

    let timestamp = Utc::now().timestamp().to_string();
    println!("timestamp: {}", timestamp);

    let nonce_str = create_nonce_str(16);
    println!("nonce_str: {}", nonce_str);

    let jsapi_ticket = get_jsapi_ticket(appid.to_string(), secret.to_string());
    println!("jsapi_ticket: {}", jsapi_ticket);

    let raw_string:String = String::from("jsapi_ticket=") + &jsapi_ticket + "&noncestr=" + &nonce_str + "&timestamp=" + &timestamp + "&url=" + &url;
    println!("raw_string: {}", raw_string);

    let mut hasher = Sha1::new();
    hasher.input_str(&raw_string);
    let signature = hasher.result_str();
    println!("SHA-1: {}", signature);

    SignPackage {
        appid: appid.to_string(),
        nonce_str: nonce_str.to_string(),
        timestamp: timestamp.to_string(),
        url: url.to_string(),
        signature: signature.to_string(),
        raw_string: raw_string.to_string(),
    }
}

fn create_nonce_str(length: i8) -> String {
    let chars = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let v: Vec<char> = chars.chars().collect();
    let mut s = String::from("");
    let mut index = 0;
    let len = chars.len();

    while index < length {
        let n = thread_rng().gen_range(0, len-1);
        s.push(v[n]);
        index += 1;
    }

    return s;
}

fn get_jsapi_ticket(appid:String, secret:String) -> String {
    let access_token = get_access_token_cache(appid.to_string(), secret.to_string());
    get_ticket_cache(access_token)
}

fn get_access_token(appid:String, secret:String) -> AccessTokenCache {
    let url:String = String::from("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=") + &appid + "&secret=" + &secret;
    println!("url: {}", url);

    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    println!("data: {:?}", data);
    let res_str = String::from_utf8(data).unwrap();
    println!("res_str: {}", res_str);
    let res_json: Value = serde_json::from_str(&res_str).unwrap();
    println!("HTTP Response: {:?}", res_json);

    set_access_token_cache(res_json["access_token"].as_str().unwrap().to_string())
}

fn set_access_token_cache(access_token:String) -> AccessTokenCache {
    let access_token_cache = AccessTokenCache {
        access_token: access_token,
        expire_time: Utc::now().timestamp()+7000,
    };
    let cache_json = serde_json::to_string(&access_token_cache).unwrap();
    match fs::write("./access_token.json", cache_json) {
        Ok(_) => println!("Write to access_token.json Successful!"),
        Err(_) => println!("Write to access_token.json Failed!"),
    };

    access_token_cache
}

fn get_access_token_cache(appid:String, secret:String) -> String {
    let data = match fs::read_to_string("./access_token.json") {
        Ok(cache_str) => {
            let cache:AccessTokenCache = serde_json::from_str(&cache_str).unwrap();
            if cache.expire_time < Utc::now().timestamp() {
                println!("Cache expired, Reload!");
                get_access_token(appid, secret)
            } else {
                cache
            }
        },
        Err(_) => get_access_token(appid, secret),
    };

    data.access_token
}

fn get_ticket(access_token:String) -> JsApiTicketCache {
    let url:String = String::from("https://api.weixin.qq.com/cgi-bin/ticket/getticket?type=jsapi&access_token=") + &String::from(access_token);
    println!("url: {}", url);

    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let res_str = String::from_utf8(data).unwrap();
    let res_json: Value = serde_json::from_str(&res_str).unwrap();
    println!("HTTP Response: {:?}", res_json);

    set_ticket_cache(res_json["ticket"].as_str().unwrap().to_string())
}

fn set_ticket_cache(ticket:String) -> JsApiTicketCache {
    let js_api_ticket_cache = JsApiTicketCache {
        ticket: ticket,
        expire_time: Utc::now().timestamp()+7000,
    };
    let cache_json = serde_json::to_string(&js_api_ticket_cache).unwrap();
    match fs::write("./jsapi_ticket.json", cache_json) {
        Ok(_) => println!("Write to jsapi_ticket.json Successful!"),
        Err(_) => println!("Write to jsapi_ticket.json Failed!"),
    };

    js_api_ticket_cache
}

fn get_ticket_cache(access_token: String) -> String {
    let data = match fs::read_to_string("./jsapi_ticket.json") {
        Ok(cache_str) => {
            let cache:JsApiTicketCache = serde_json::from_str(&cache_str).unwrap();
            if cache.expire_time < Utc::now().timestamp() {
                println!("Cache expired, Reload!");
                get_ticket(access_token)
            } else {
                cache
            }
        },
        Err(_) => get_ticket(access_token),
    };

    data.ticket
}
