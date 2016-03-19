use std::collections::BTreeMap;
use std::sync::mpsc;
use std::thread;

use rustc_serialize::json;
use rustc_serialize::json::{Json, ToJson};

use websocket::{Server, Message, Sender, Receiver, WebSocketStream};
use websocket::server::{sender, receiver};
use websocket::message::Type as MessageType;

use phys::area::Area;
use user::User;

pub fn start () {

    let address = "127.0.0.1:1337";

    let server = Server::bind(address).unwrap();

    println!("Listening to {}", address);

    for connection in server {

        thread::spawn(move || {

            println!("New connection");

            let request = connection.unwrap().read_request().unwrap();
            let response = request.accept();
            let client = response.send().unwrap();

            let (sender, receiver) = client.split();

            serve(sender, receiver);
        });
    }
}

fn serve(mut sender: sender::Sender<WebSocketStream>, mut receiver: receiver::Receiver<WebSocketStream>) {

    let mut response = BTreeMap::new();
    response.insert("message".to_string(), "Welcome!".to_json());
    let response = Json::Object(response);
    let response = json::encode(&response).unwrap();
    sender.send_message(&Message::text(response)).unwrap();

    let (action_sender, action_receiver) = mpsc::channel();

    thread::spawn(move || {
        for message in receiver.incoming_messages() {

            let message = message.unwrap();

            let action = extract_action(message);

            if action.is_some() {

                let action = action.unwrap();

                println!("Received action: {:?}", action);

                let action_sender = action_sender.clone();
                action_sender.send(action).unwrap();
            }
        }
    });

    let user = User::new(action_receiver, sender);
    let mut area = Area::new(user);

    loop {
        area.tick();
    }
}

fn extract_action(message: Message) -> Option<(String, String)> {
    match message.opcode {
        MessageType::Text => extract_json_action(String::from_utf8(message.payload.into_owned()).unwrap()),
        _ => Option::None,
    }
}

fn extract_json_action(string: String) -> Option<(String, String)> {

    let json = Json::String(string);
    let action = json.find("action");

    if action.is_none() {
        return Option::None;
    }

    let action = action.unwrap();

    let action_type = action.find("type");
    let resolver = action.find("resolver");

    if action_type.is_none() || resolver.is_none() {
        return Option::None;
    }

    Option::Some((json::encode(action_type.unwrap()).unwrap(), json::encode(resolver.unwrap()).unwrap()))
}
