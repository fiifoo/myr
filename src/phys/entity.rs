use std::collections::HashMap;
use phys::area::Tile;

pub struct EntityManager {
    entities: Vec<Entity>,
    map: HashMap<i64, usize>,
    max_id: i64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Entity {
    pub name: String,
    pub id: i64,
    pub tile: Tile,
    pub damage: i64,
    pub radiation: i64,
    pub tick: i64
}

impl EntityManager {

    pub fn new() -> EntityManager {
        EntityManager {map: HashMap::new(), entities: vec![], max_id: 0}
    }

    pub fn add(&mut self, entity: Entity) -> &mut EntityManager {
        let index = self.entities.len();

        self.map.insert(entity.id, index);
        self.entities.push(entity);

        self
    }

    pub fn get(&self, id: i64) -> &Entity {
        let index = self.map.get(&id).unwrap();
        &self.entities.get(*index).unwrap()
    }

    pub fn get_mut(&mut self, id: i64) -> &mut Entity {
        let index = self.map.get(&id).unwrap();
        self.entities.get_mut(*index).unwrap()
    }

    pub fn get_all(&self) -> &Vec<Entity> {
        &self.entities
    }

    fn generate_id(&mut self) -> i64 {
        self.max_id += 1;

        self.max_id
    }
}

impl Entity {

    pub fn new (manager: &mut EntityManager, name: String, tile: Tile) -> i64 {

        let id = manager.generate_id();
        let entity = Entity {name: name, tile: tile, id: id, tick: 0, damage: 0, radiation: 0};

        manager.add(entity);

        id
    }

    pub fn dump(&self) {
        println!("name = {}, tile = ({},{}), damage = {}, radiation = {}", self.name, self.tile.x, self.tile.y, self.damage, self.radiation);
    }
}
