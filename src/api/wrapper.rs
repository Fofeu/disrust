//MIGHT WANT TO SEPERATE PRIVATE AND PUBLIC FOR CLARITY AND NEATNESS
//Make everything return results

use crate::{api::data::*, ui::channels::App};
use serde_json::Value;
use std::string::String;

fn get_length(list: &serde_json::Value) -> usize {
    match list.as_array() {
        Some(v) => v.len(),
        None => panic!("TRIED TO GET LENGTH OF AN EMPTY RESPONSE"),
    }
}

//Should be removed
//Would be much better to figure out deserialization in structs
fn get(list: &serde_json::Value, index: usize, key: &str) -> anyhow::Result<String, &'static str> {
    //BIG BRAIN DOWN HERE

    match list[index].get(key) {
        Some(v) => match v {
            Value::Number(v) => Ok(v.to_string()),
            Value::String(v) => Ok(v.to_string()),
            _ => Err("bruh"),
        },
        None => Err("bruh"),
    }
}

//Get a reqwest and return json
fn request_json(conn: &Connection, url: &str) -> serde_json::Value {
    //request discord data
    let auth = &conn.auth;
    let client = &conn.client;

    let response: serde_json::Value = client
        .get(url)
        .header(&auth.0, &auth.1)
        .send()
        .expect("Shit out of luck")
        .json()
        .unwrap();

    // dbg!(&response);
    return response;
}

pub fn messages(conn: &Connection, channel: &Channel) -> anyhow::Result<Vec<Msg>, &'static str> {
    let url = format!(
        "https://discord.com/api/v9/channels/{}/messages?limit=80",
        channel.id
    );
    let response = request_json(conn, url.as_str());

    //delete this last get
    let potential_panic = get(&response, 0, "code");

    match potential_panic {
        Ok(_) => return Err("ACCESS DENIED"),
        Err(_) => (),
    }

    let mut message_list = Vec::new();

    let len = get_length(&response);
    for i in 0..len {
        let msg = Msg::from(&response[i]);
        //RETURNS MESSAGES IN REVERSE
        message_list.push(msg);
    }
    message_list.reverse(); //fixes reverse order messages
    return Ok(message_list);
}

pub fn send_message(app: &mut App, input: &String) {
    let channel_id = app.get_channel().id;
    let conn = &app.conn;
    let client = &conn.client;
    let header = conn.auth.clone();

    let params = [("content", input)];

    let url = format!(
        "https://discord.com/api/v9/channels/{}/messages",
        channel_id
    );
    let _response = client
        .post(url)
        .header(header.0, header.1)
        .form(&params)
        .send()
        .expect("Failed to send input");
}
