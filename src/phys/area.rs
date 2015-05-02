use phys::action::Action;
use phys::action::Move;
use phys::action::Target;
use phys::entity::EntityManager;
use phys::entity::Entity;

pub struct Area {
    manager: EntityManager,
    tick: i32
}

impl Area {

    pub fn new () -> Area {

        let mut manager = EntityManager::new();

        Entity::new(&mut manager, "Entity 1".to_string(), (0,0));
        Entity::new(&mut manager, "Entity 2".to_string(), (10,10));
        Entity::new(&mut manager, "Entity 3".to_string(), (100,100));

        let area = Area {manager: manager, tick: 0};

        area
    }

    pub fn tick(&mut self) {

        let id = self.find_tick_entity_id();

        match id {
            Option::Some(id) => self.tick_entity(id),
            Option::None => self.tick += 1,
        }
    }

    fn find_tick_entity_id(&self) -> Option<usize> {

        for entity in self.manager.get_all() {
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

    fn decide_action(&self, entity_id: usize) -> Box<Action> {

        let entity = self.manager.get(entity_id);

        let target = Target::Tile(entity.tile.0 + 1, entity.tile.1 + 1);
        let action = Move {entity_id: entity.id, target: target};

        Box::new(action)
    }

    fn mutate(&mut self, action: Box<Action>) {
        action.resolve(&mut self.manager);
    }
}

fn dump_entity(entity: &Entity) {
    let (x,y) = entity.tile;
    println!("{}, {}, {}", entity.name, x, y);
}
