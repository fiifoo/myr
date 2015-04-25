use std::collections::HashMap;

pub struct Area {
    manager: EntityManager,
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
}

struct Action {
    entity: usize,
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

    pub fn foo(&mut self) {

        let actions = self.turn();

        self.mutate(actions);

        dump_entity(&self.manager.get(1));
    }

    fn turn(&self) -> Vec<Action> {

        let resolve = Box::new(|entity: &mut Entity| {
            entity.tile = (10,10);
        });

        let action = Action {entity: 1, resolve: resolve};

        vec!(action)
    }

    fn mutate(&mut self, actions: Vec<Action>) {
        let action = &actions[0];

        let entity = self.manager.get_mut(action.entity);
        let resolve = &action.resolve;

        resolve(entity);
        //entity.tile = (12,13);
    }
}

pub fn create () -> Area {

    let mut area = Area {manager: EntityManager {map: HashMap::new(), entities: vec![], max_id: 0}};
    let id = area.manager.generate_id();

    area.manager.add(Entity {name: "Entity 1".to_string(), tile: (1,2), id: id});

    area
}

fn dump_entity(entity: &Entity) {
    let (x,y) = entity.tile;
    println!("{}, {}, {}", entity.name, x, y);
}
