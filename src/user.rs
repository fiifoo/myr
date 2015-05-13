use std::sync::mpsc;

use phys::action::Action;
use phys::action::ActionType;

pub struct User {
    action_receiver: mpsc::Receiver<(String, String)>,
}

impl User {

    pub fn new (action_receiver: mpsc::Receiver<(String, String)>) -> User {
        User {action_receiver: action_receiver}
    }

    pub fn get_action(&self, entity_id: i64) -> Action {

        let (action_type, resolver) = self.action_receiver.recv().unwrap();

        let action_type = ActionType::new_from_json(action_type);
        let resolver = action_type.new_resolver_from_json(resolver);

        Action {entity_id: entity_id, resolver: resolver}
    }
}
