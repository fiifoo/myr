use phys::action::Action;
use phys::action::Move;
use phys::action::Target;
use phys::entity::Entity;
use phys::entity::EntityId;
use phys::entity::EntityManager;

pub struct Area {
    manager: EntityManager,
    tick: i32
}

pub struct Tile(pub i32, pub i32);

impl Area {

    pub fn new () -> Area {

        let mut manager = EntityManager::new();

        Entity::new(&mut manager, "Entity 1".to_string(), Tile(0,0));
        Entity::new(&mut manager, "Entity 2".to_string(), Tile(10,10));
        Entity::new(&mut manager, "Entity 3".to_string(), Tile(100,100));

        let area = Area {manager: manager, tick: 0};

        area
    }

    pub fn tick(&mut self) {

        let id = self.find_tick_entity();

        match id {
            Option::Some(id) => self.tick_entity(id),
            Option::None => self.tick += 1,
        }
    }

    fn find_tick_entity(&self) -> Option<EntityId> {

        for entity in self.manager.get_all() {
            if entity.tick == self.tick {
                return Option::Some(entity.id.clone());
            }
        }

        Option::None
    }

    fn tick_entity(&mut self, id: EntityId) {

        let dump_id = id.clone();

        let action = self.decide_action(id);
        self.mutate(action);

        dump_entity(&self.manager.get(dump_id));
    }

    fn decide_action(&self, id: EntityId) -> Box<Action> {

        let entity = self.manager.get(id);

        let Tile(x, y) = entity.tile;

        let target = Target::Tile(Tile(x + 1, y + 1));
        let action = Move {entity: entity.id.clone(), target: target};

        Box::new(action)
    }

    fn mutate(&mut self, action: Box<Action>) {
        action.resolve(&mut self.manager);
    }
}

impl Clone for Tile {
    fn clone (&self) -> Self {
        let Tile(x, y) = *self;
        Tile(x, y)
    }
}

fn dump_entity(entity: &Entity) {
    let Tile(x,y) = entity.tile;
    println!("{}, {}, {}", entity.name, x, y);
}
