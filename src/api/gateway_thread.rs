/*
op codes cheat sheet:
0: gateway event
1: heartbeat sent
2: ready event (A load of info like guilds, user, settings, etc)
10: discord sent you heartbeat interval, hello
11: discord received your heartbeat

The gateway events are identified by string names
VC has its own op codes

btw people's email address is public through the api I think, weird
 */

use std::time::Duration;

use serde_json::{self, Value};
use url::Url;

use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::{select_biased, stream};
use futures_util::StreamExt;

use tokio::net::TcpStream;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::api::consts::GATEWAY_URL;
use crate::model::event::{ConnectionProperties, Event};

pub async fn start_thread(token: &String) -> anyhow::Result<()> {
    // let (_tx, _rx) = mpsc::unbounded();

    let (mut socket, _response) = connect_async(Url::parse(GATEWAY_URL).unwrap()).await?;

    let heartbeat_interval = heartbeat_handshake(&mut socket).await?;
    //ticks whenever we should send a heartbeat
    let mut heartbeater = IntervalStream::new(interval(heartbeat_interval.clone())).fuse();

    identify_handshake(&mut socket, token).await?;

    let main_sender =
    {
        let last_sequence_number: Option<u64> = None;
        let mut heartbeat_ack = true;
        tokio::spawn(async move {
            loop
            {
                select_biased! {
                    _ = heartbeater.next() =>
                    {
                        if heartbeat_ack {
                            match socket.send(Event::heartbeat(last_sequence_number).to_ws_msg()).await {
                                Ok(()) => {println!(".");heartbeat_ack = false;},
                                Err (e) => todo!("Couldn't send heartbeat {:?}", e),
                            }
                        } else {
                            todo!("Gateway never replied to our heartbeat, we should disconnect and reconnect")
                        }
                    },
                    msg = socket.next() =>
                    {
                        if let Some(Ok(Message::Text(s))) = msg {
                            let j : Value = serde_json::from_str(&s).unwrap();
                            let e = Event::try_from(j).unwrap();
                            use Event::*;
                            match e {
                                HeartbeatACK => heartbeat_ack = true,
                                Heartbeat(_) => todo!(),
                                Hello (_) | Identify (_) | Ready(_) => unreachable!("{:?}", e),
                            }
                        }
                        else
                        {
                            todo!()
                        }
                    },
                }
            };
        })
    }.await?;

    // let hello_event = recv_hello_event(&mut socket).await?;

    // //Not sure if it's correct terminology
    // let handshake = read_json_event(&mut socket).await?;
    // print!("{}\n", serde_json::to_string_pretty(&handshake)?);
    // let hb_interval = handshake["d"]["heartbeat_interval"].as_i64().unwrap();
    // println!("Received Hbeat: {}", hb_interval);

    // identify(&mut socket, token);

    // //Can get a lot of data from it in order to
    // //not update much in network_thread
    // let ready = read_json_event(&mut socket).await?;
    // ready_event(&tx, ready);

    // tokio::spawn(async move {
    //     let mut timer = Instant::now();
    //     loop {
    //         let event = read_json_event(&mut socket).await;
    //         // dbg!(&event);
    //         match &event {
    //             Ok(event) => {
    //                 let op_code = event["op"].as_i64().unwrap();
    //                 match op_code {
    //                     1 => heartbeat(&mut socket).await,
    //                     0 => {
    //                         let event_name = event["t"].as_str().unwrap();
    //                         match event_name {
    //                             "MESSAGE_CREATE" => message_created(&tx, &event),
    //                             "MESSAGE_REACTION_ADD" => (),
    //                             "MESSAGE_REACTION_REMOVE" => (),
    //                             "TYPING_START" => (),
    //                             "CHANNEL_CREATE" => (),
    //                             "GUILD_CREATE" => (),
    //                             "GUILD_DELETE" => (),
    //                             _ => (),
    //                         }
    //                     }
    //                     _ => () // Unhandled (unknown ?) op_code

    //                 }
    //             }
    //             Err(_) => {
    //                 println!("Gateway disconnected");
    //                 continue;
    //             }
    //         };

    //         //Heartbeat here
    //         //A thread would have to borrow the socket and it was a pain
    //         let elapsed = timer.elapsed().as_millis() as i64;
    //         if hb_interval <= elapsed {
    //             heartbeat(&mut socket);
    //             timer = Instant::now();
    //         }
    //     }
    // });

    Ok(())
}

async fn heartbeat_handshake(
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> anyhow::Result<Duration> {
    match socket.next().await {
        Some(Ok(Message::Text(s))) => {
            let json : Value = serde_json::from_str(&s)?;
            let event = Event::try_from(json)?;
            match event {
                Event::Hello(hb) => Ok(hb.heartbeat_interval),
                _ => unreachable!(),
            }
        }
        None => todo!(),
        _ => todo!(),
    }
}

async fn identify_handshake(socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, token: &String) -> anyhow::Result<()> {
    socket.send(Event::identify(token.clone(), ConnectionProperties::default(), 50).to_ws_msg()).await?;
    match socket.next().await {
        Some(Ok(Message::Text(s))) =>
            {
                let json : Value = serde_json::from_str(&s)?;
                let event = Event::try_from(json)?;
                match event {
                    Event::Ready(_) => todo!(),
                    _ => unreachable!(),
                }
            },
        None => todo!(),
        _ => todo!()
    }
}

// //Each event has an attached sequence number
// //Heartbeats need to include latest sequence number
// //^^^ Didn't use it in python test and had no problems. Abandoned for now
// async fn heartbeat(socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>) {
//     let reply = Message::Text(
//         r#"{
//         "op": 1,
//         "d": "null"
//     }"#
//         .into(),
//     );

//     // pin_mut!(socket).start_send(reply)
// }

// fn identify(socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>, token: &str) {
//     //ugly as fuck
//     let reply = format!(
//         "{{
//         \"op\": 2,
//         \"d\": {{
//             \"token\": \"{}\",
//             \"properties\": {{
//                 \"$os\": \"linux\",
//                 \"$browser\": \"chrome\",
//                 \"$device\": \"pc\"
//             }}
//         }}
//     }}",
//         token
//     );

//     let reply = Message::Text(reply.into());

//     // socket.write_message(reply).expect("Identification failed");
// }

// //Makes a Msg object and sends it back to ui thread
// fn message_created(tx: &mpsc::Sender<GatewayResponse>, event: &Value) {
//     let msg = Msg::from(&event["d"]);
//     let gate_response = GatewayResponse::msg_create(msg);
//     tx.send(gate_response).unwrap();
// }

// fn ready_event(tx: &mpsc::Sender<GatewayResponse>, event: Value) {
//     let guilds = Guild::from_list(&event["d"]);
//     let gate_response = GatewayResponse::ready(guilds);
//     tx.send(gate_response).unwrap();
// }

// // use result instead
// // Some weird shit with gateway disconnect idk
// async fn read_json_event(
//     socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
// ) -> anyhow::Result<Event> {
//     let msg = socket.next().await;
//     match msg {
//         Some (Ok (Message::Text(s))) =>
//             {
//                 let json = serde_json::from_str::<serde_json::Value>(&s)?;
//                 Ok(Event::try_from(json)?)
//             },
//         _ => todo!(),
//     }
//     // let text_msg = msg.to_text().expect("No text, I think");
//     // let json_msg = serde_json::from_str(text_msg);

//     // match json_msg {
//     //     Ok(v) => Ok(v),
//     //     Err(v) => Err(v.into()),
//     // }
// }
