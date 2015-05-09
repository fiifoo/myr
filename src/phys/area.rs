use ai;
use phys::entity::Entity;
use phys::entity::EntityManager;

pub struct Area {
    manager: EntityManager,
    tick: i64
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Tile {
    pub x: i64,
    pub y: i64,
}

impl Area {

    pub fn new () -> Area {

        let mut manager = EntityManager::new();

        Entity::new(&mut manager, "Entity 1".to_string(), Tile {x: 0, y: 0});
        Entity::new(&mut manager, "Entity 2".to_string(), Tile {x: 10, y: 10});
        Entity::new(&mut manager, "Entity 3".to_string(), Tile {x: 100, y: 100});

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

    fn find_tick_entity(&self) -> Option<i64> {

        for entity in self.manager.get_all() {
            if entity.tick == self.tick {
                return Option::Some(entity.id);
            }
        }

        Option::None
    }

    fn tick_entity(&mut self, id: i64) {

        let action = ai::decide(self.manager.get(id));
        let effects = action.resolve(&self.manager);

        for effect in effects {
            let effect_entity = self.manager.get_mut(effect.entity_id);
            effect.resolve(effect_entity);
        }

        let entity = self.manager.get_mut(action.entity_id);
        entity.tick += 1;

        entity.dump();
    }
}

impl Clone for Tile {
    fn clone (&self) -> Self {
        Tile {x: self.x, y: self.y}
    }
}
