use phys::action;
use phys::action::Action;
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

        let action = self.decide_action(id.clone());
        let effects = action.resolve(&self.manager);

        for effect in effects {
            let effect_entity = self.manager.get_mut(effect.entity.clone());
            effect.resolve(effect_entity);
        }

        let entity = self.manager.get_mut(action.entity.clone());
        entity.tick += 1;

        dump_entity(entity);
    }

    fn decide_action(&self, id: EntityId) -> Action {

        let entity = self.manager.get(id);

        let Tile(x, y) = entity.tile;

        let target = action::Target::Tile(Tile(x + 1, y + 1));
        let resolver = Box::new(action::MoveResolver {target: target});
        let action = Action {entity: entity.id.clone(), resolver: resolver};

        action
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
