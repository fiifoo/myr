use std::collections::HashMap;
use phys::area::Tile;

pub struct EntityManager {
    map: HashMap<usize, usize>,
    entities: Vec<Entity>,
    max_id: usize,
}

pub struct Entity {
    pub name: String,
    pub id: EntityId,
    pub tile: Tile,
    pub tick: i32,
    pub damage: i32,
    pub radiation: i32
}

pub struct EntityId(usize);

impl EntityManager {

    pub fn new() -> EntityManager {
        EntityManager {map: HashMap::new(), entities: vec![], max_id: 0}
    }

    pub fn add(&mut self, entity: Entity) -> &mut EntityManager {
        let EntityId(id_value) = entity.id;
        let index = self.entities.len();

        self.map.insert(id_value, index);
        self.entities.push(entity);

        self
    }

    pub fn get(&self, id: EntityId) -> &Entity {
        let EntityId(id_value) = id;
        let index = self.map.get(&id_value).unwrap();
        &self.entities.get(*index).unwrap()
    }

    pub fn get_mut(&mut self, id: EntityId) -> &mut Entity {
        let EntityId(id_value) = id;
        let index = self.map.get(&id_value).unwrap();
        self.entities.get_mut(*index).unwrap()
    }

    pub fn get_all(&self) -> &Vec<Entity> {
        &self.entities
    }

    fn generate_id(&mut self) -> EntityId {
        self.max_id += 1;

        EntityId(self.max_id)
    }
}

impl Entity {

    pub fn new (manager: &mut EntityManager, name: String, tile: Tile) -> EntityId {

        let id = manager.generate_id();
        let entity = Entity {name: name, tile: tile, id: id.clone(), tick: 0, damage: 0, radiation: 0};

        manager.add(entity);

        id
    }

    pub fn dump(&self) {
        let Tile(x,y) = self.tile;
        println!("name = {}, tile = ({},{}), damage = {}, radiation = {}", self.name, x, y, self.damage, self.radiation);
    }
}

impl Clone for EntityId {
    fn clone (&self) -> Self {
        let EntityId(value) = *self;
        EntityId(value)
    }
}
