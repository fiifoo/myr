use std::collections::HashMap;

pub struct Area {
    manager: EntityManager,
    tick: i32
}

struct EntityManager {
    map: HashMap<usize, usize>,
    entities: Vec<Entity>,
    max_id: usize,
}

struct Entity {
    name: String,
    id: usize,
    tile: (i32, i32),
    tick: i32
}

struct Action {
    entity_id: usize,
    resolve: Box<Fn (&mut Entity)>,
}

impl EntityManager {

    fn generate_id(&mut self) -> usize {
        self.max_id += 1;

        self.max_id
    }

    fn add(&mut self, entity: Entity) -> &mut EntityManager {
        let index = self.entities.len();

        self.map.insert(entity.id, index);
        self.entities.push(entity);

        self
    }

    fn get(&self, id: usize) -> &Entity {
        let index = self.map.get(&id).unwrap();
        &self.entities.get(*index).unwrap()
    }

    fn get_mut(&mut self, id: usize) -> &mut Entity {
        let index = self.map.get(&id).unwrap();
        self.entities.get_mut(*index).unwrap()
    }
}

impl Area {

    pub fn tick(&mut self) {

        let id = self.find_tick_entity_id();

        match id {
            Option::Some(id) => self.tick_entity(id),
            Option::None => self.tick += 1,
        }
    }

    fn find_tick_entity_id(&self) -> Option<usize> {

        for entity in &self.manager.entities {
            if entity.tick == self.tick {
                return Option::Some(entity.id);
            }
        }

        Option::None
    }

    fn tick_entity(&mut self, id: usize) {

        let action = self.decide_action(id);
        self.mutate(action);

        dump_entity(&self.manager.get(id));
    }

    fn decide_action(&self, entity_id: usize) -> Action {

        let resolve = Box::new(|entity: &mut Entity| {
            entity.tile = (entity.tile.0 + 1, entity.tile.1 + 1);
        });

        let action = Action {entity_id: entity_id, resolve: resolve};

        action
    }

    fn mutate(&mut self, action: Action) {

        let entity = self.manager.get_mut(action.entity_id);
        let resolve = &action.resolve;

        resolve(entity);
        entity.tick += 1;
    }
}

pub fn create () -> Area {

    let manager = EntityManager {map: HashMap::new(), entities: vec![], max_id: 0};
    let mut area = Area {manager: manager, tick: 0};

    let entity_id = area.manager.generate_id();
    let entity = Entity {name: "Entity 1".to_string(), tile: (0,0), id: entity_id, tick: 0};
    area.manager.add(entity);

    let other_entity_id = area.manager.generate_id();
    let other_entity = Entity {name: "Entity 2".to_string(), tile: (10,10), id: other_entity_id, tick: 0};
    area.manager.add(other_entity);

    area
}

fn dump_entity(entity: &Entity) {
    let (x,y) = entity.tile;
    println!("{}, {}, {}", entity.name, x, y);
}
