
use std::collections::BTreeMap;
use std::sync::mpsc;
use std::thread;

use rustc_serialize::json;
use rustc_serialize::json::{Json, ToJson};

use websocket::{Server, Message, Sender, Receiver, WebSocketStream};
use websocket::server::{sender, receiver};

use phys::area::Area;

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

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for message in receiver.incoming_messages::<Message>() {

            let message = message.unwrap();

            let command = extract_command(message);

            if command.is_some() {

                let command = command.unwrap();

                println!("Received command: {}", command);

                let mut response = BTreeMap::new();
                response.insert("received_command".to_string(), command.to_json());
                let response = Json::Object(response);
                let response = json::encode(&response).unwrap();
                sender.send_message(Message::Text(response)).unwrap();

                let tx = tx.clone();
                tx.send(command).unwrap();
            }

        }
    });

    let mut area = Area::new();

    loop {
        Option::Some(rx.recv().unwrap());
        area.tick();
    }
}

fn extract_command(message: Message) -> Option<i64> {

    match message {
        Message::Text(string) => extract_json_command(string),
        _ => Option::None,
    }
}

fn extract_json_command(string: String) -> Option<i64> {

    let json = Json::from_str(string.as_str()).unwrap();
    let json = json.as_object().unwrap();
    let command = json.get("command").unwrap();

    Option::Some(command.as_i64().unwrap())
}
