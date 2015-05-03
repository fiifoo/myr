use phys::area::Tile;
use phys::entity::EntityId;
use phys::entity::EntityManager;

pub trait Effect {
    fn resolve(&self, &mut EntityManager);
}

pub struct Move {
    pub entity: EntityId,
    pub tile: Tile,
}

impl Effect for Move {
    fn resolve(&self, manager: &mut EntityManager) {
        let entity = manager.get_mut(self.entity.clone());
        entity.tile = self.tile.clone();
    }
}
