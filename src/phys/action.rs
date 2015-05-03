use phys::area::Tile;
use phys::effect;
use phys::effect::Effect;
use phys::entity::EntityId;
use phys::entity::EntityManager;

pub trait Action {
    fn resolve(&self, &EntityManager) -> Vec<Box<Effect>>;
}

pub struct Move {
    pub entity: EntityId,
    pub target: Target,
}

impl Action for Move {
    fn resolve(&self, manager: &EntityManager) -> Vec<Box<Effect>> {

        let tile = match self.target {
            Target::Tile(ref tile) => tile.clone(),
            _ => panic!("Tile required as target."),
        };

        let effect = effect::Move {entity: self.entity.clone(), tile: tile};
        vec![Box::new(effect)]
    }
}

pub enum Target {
    Entity(EntityId),
    Tile(Tile),
    None,
}
