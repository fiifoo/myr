use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::Decodable;
use rustc_serialize::Decoder;

use phys::area::Tile;
use phys::effect;
use phys::effect::Effect;
use phys::entity::Entity;
use phys::entity::EntityManager;

pub struct Action {
    pub entity_id: i64,
    pub resolver: Box<ActionResolver>,
}

#[derive(RustcDecodable)]
pub enum ActionType {
    Attack,
    Move,
    NukeEm,
}

impl ActionType {
    pub fn new_from_json(json: String) -> ActionType {

        let json = Json::from_str(json.as_str()).unwrap();
        let mut decoder = json::Decoder::new(json);

        ActionType::decode(&mut decoder).unwrap()
    }
    pub fn new_resolver_from_json(&self, json: String) -> Box<ActionResolver> {

        let json = Json::from_str(json.as_str()).unwrap();
        let mut decoder = json::Decoder::new(json);

        match *self {
            ActionType::Attack => Box::new(AttackResolver::decode(&mut decoder).unwrap()),
            ActionType::Move => Box::new(MoveResolver::decode(&mut decoder).unwrap()),
            ActionType::NukeEm => Box::new(NukeEmResolver::decode(&mut decoder).unwrap()),
        }
    }
}

pub trait ActionResolver {
    fn resolve(&self, &Entity, &EntityManager) -> Vec<Effect>;
}

#[derive(RustcDecodable)]
pub struct AttackResolver {
    pub target: Target,
}

#[derive(RustcDecodable)]
pub struct MoveResolver {
    pub tile: Tile,
}

#[derive(RustcDecodable)]
pub struct NukeEmResolver;

#[derive(RustcDecodable)]
pub enum Target {
    Entity(i64),
    Tile(Tile),
    None,
}

impl Action {
    pub fn resolve(&self, manager: &EntityManager) -> Vec<Effect> {
        let entity = manager.get(self.entity_id);
        self.resolver.resolve(entity, manager)
    }
}

impl ActionResolver for AttackResolver {
    #[allow(unused_variables)]
    fn resolve(&self, entity: &Entity, manager: &EntityManager) -> Vec<Effect> {

        let target_id = match self.target {
            Target::Entity(entity_id) => entity_id,
            _ => panic!("Only entity supported as target for now."),
        };

        let resolver = Box::new(effect::DamageResolver {damage: 1});
        let effect = Effect {entity_id: target_id, resolver: resolver};

        vec![effect]
    }
}

impl ActionResolver for MoveResolver {
    #[allow(unused_variables)]
    fn resolve(&self, entity: &Entity, manager: &EntityManager) -> Vec<Effect> {

        let resolver = Box::new(effect::MovementResolver {tile: self.tile.clone()});
        let effect = Effect {entity_id: entity.id, resolver: resolver};

        vec![effect]
    }
}

impl ActionResolver for NukeEmResolver {
    #[allow(unused_variables)]
    fn resolve(&self, entity: &Entity, manager: &EntityManager) -> Vec<Effect> {

        let mut effects = vec![];

        for entity in manager.get_all() {
            let resolver = Box::new(effect::DamageResolver {damage: 999});
            let effect = Effect {entity_id: entity.id, resolver: resolver};
            effects.push(effect);
        }
        for entity in manager.get_all() {
            let resolver = Box::new(effect::RadiationResolver {radiation: 999});
            let effect = Effect {entity_id: entity.id, resolver: resolver};
            effects.push(effect);
        }

        effects
    }
}
