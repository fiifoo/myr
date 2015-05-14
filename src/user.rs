use std::collections::BTreeMap;
use std::sync::mpsc;

use rustc_serialize::json;
use rustc_serialize::json::Json;

use websocket::{Sender, Message, WebSocketStream};
use websocket::server::sender;

use phys::action::Action;
use phys::action::ActionType;
use phys::entity::Entity;

pub struct User {
    action_receiver: mpsc::Receiver<(String, String)>,
    sender: sender::Sender<WebSocketStream>,
}

impl User {

    pub fn new (action_receiver: mpsc::Receiver<(String, String)>, sender: sender::Sender<WebSocketStream>) -> User {
        User {action_receiver: action_receiver, sender: sender}
    }

    pub fn get_action(&self, entity_id: i64) -> Action {

        let (action_type, resolver) = self.action_receiver.recv().unwrap();

        let action_type = ActionType::new_from_json(action_type);
        let resolver = action_type.new_resolver_from_json(resolver);

        Action {entity_id: entity_id, resolver: resolver}
    }

    pub fn send_entities(&mut self, entities: &Vec<Entity>) {

        let json = json::encode(entities).unwrap();

        let mut response = BTreeMap::new();
        response.insert("entities".to_string(), Json::from_str(json.as_str()).unwrap());
        let response = Json::Object(response);
        let response = json::encode(&response).unwrap();
        self.sender.send_message(Message::Text(response)).unwrap();
    }
}
