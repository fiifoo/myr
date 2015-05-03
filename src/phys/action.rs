use phys::area::Tile;
use phys::effect;
use phys::effect::Effect;
use phys::entity::Entity;
use phys::entity::EntityId;
use phys::entity::EntityManager;

pub struct Action {
    pub entity: EntityId,
    pub resolver: Box<ActionResolver>,
}

impl Action {
    pub fn resolve(&self, manager: &EntityManager) -> Vec<Effect> {
        let entity = manager.get(self.entity.clone());
        self.resolver.resolve(entity, manager)
    }
}

pub trait ActionResolver {
    fn resolve(&self, &Entity, &EntityManager) -> Vec<Effect>;
}

pub struct MoveResolver {
    pub target: Target,
}

impl ActionResolver for MoveResolver {
    fn resolve(&self, entity: &Entity, manager: &EntityManager) -> Vec<Effect> {

        let tile = match self.target {
            Target::Tile(ref tile) => tile.clone(),
            _ => panic!("Tile required as target."),
        };

        let resolver = Box::new(effect::MoveResolver {tile: tile});
        let effect = Effect {entity: entity.id.clone(), resolver: resolver};

        vec![effect]
    }
}

pub enum Target {
    Entity(EntityId),
    Tile(Tile),
    None,
}
