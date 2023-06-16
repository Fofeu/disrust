//MIGHT WANT TO SEPERATE PRIVATE AND PUBLIC FOR CLARITY AND NEATNESS
//Make everything return results

// use crate::api::consts::AUTH_KEY;
// use serde_json::Value;
// use std::string::String;

// /// Using Connection conn, try loading messages of Channel channel
// pub async fn messages(conn: &Connection, chan: &Channel) -> anyhow::Result<Vec<Msg>> {
//     let url = format!(
//         "https://discord.com/api/v9/channels/{}/messages?limit=80",
//         chan.id
//     );
//     let response: Value =
//         conn.client
//         .get(url)
//         .header(AUTH_KEY, &conn.token)
//         .send()
//         .await?
//         .json()
//         .await?;

//     //We should be able to get rid of that unwrap somehow
//     //Probably by inspecting the JSON we received and validating that it is in the expected form
//     let v  = response.as_array().unwrap();
//     Ok(v.into_iter().rev().map(Msg::from).collect())
// }

// /// Using Connection conn, send on channel chan a message composed of the String input
// pub async fn send_message(conn: &Connection, chan: &Channel, input: &String) -> Result<reqwest::Response, reqwest::Error> {
//     let channel_id = &chan.id;
//     let client = &conn.client;

//     let params = [("content", input)];

//     let url = format!(
//         "https://discord.com/api/v9/channels/{}/messages",
//         channel_id
//     );
//     client
//         .post(url)
//         .header(AUTH_KEY, &conn.token)
//         .form(&params)
//         .send()
//         //not sure why Rust forces me to await here
//         .await
// }
