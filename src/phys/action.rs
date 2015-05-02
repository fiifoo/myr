use phys::entity::EntityManager;

pub trait Action {
    fn resolve(&self, &mut EntityManager);
}

pub struct Move {
    pub entity_id: usize,
    pub target: Target,
}

impl Action for Move {
    fn resolve(&self, manager: &mut EntityManager) {

            let entity = manager.get_mut(self.entity_id);

            match self.target {
                Target::Tile(x, y) => entity.tile = (x, y),
                _ => (),
            }

            entity.tick += 1;
    }
}

pub enum Target {
    Entity(usize),
    Tile(i32, i32),
    None,
}
