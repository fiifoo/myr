use std::collections::HashMap;

pub struct EntityManager {
    map: HashMap<usize, usize>,
    entities: Vec<Entity>,
    max_id: usize,
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

    pub fn get(&self, id: usize) -> &Entity {
        let index = self.map.get(&id).unwrap();
        &self.entities.get(*index).unwrap()
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Entity {
        let index = self.map.get(&id).unwrap();
        self.entities.get_mut(*index).unwrap()
    }

    pub fn get_all(&self) -> &Vec<Entity> {
        &self.entities
    }

    fn generate_id(&mut self) -> usize {
        self.max_id += 1;

        self.max_id
    }
}

pub struct Entity {
    pub name: String,
    pub id: usize,
    pub tile: (i32, i32),
    pub tick: i32
}

impl Entity {

    pub fn new (manager: &mut EntityManager, name: String, tile: (i32, i32)) -> usize {

        let id = manager.generate_id();
        let entity = Entity {name: name, tile: tile, id: id, tick: 0};

        manager.add(entity);

        id
    }
}
