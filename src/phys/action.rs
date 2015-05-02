use phys::area::Tile;
use phys::entity::EntityId;
use phys::entity::EntityManager;

pub trait Action {
    fn resolve(&self, &mut EntityManager);
}

pub struct Move {
    pub entity: EntityId,
    pub target: Target,
}

impl Action for Move {
    fn resolve(&self, manager: &mut EntityManager) {

            let entity = manager.get_mut(self.entity.clone());

            match self.target {
                Target::Tile(ref tile) => entity.tile = tile.clone(),
                _ => (),
            }

            entity.tick += 1;
    }
}

pub enum Target {
    Entity(EntityId),
    Tile(Tile),
    None,
}
