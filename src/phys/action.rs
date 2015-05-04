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
    pub tile: Tile,
}

pub struct AttackResolver {
    pub target: Target,
}

pub struct NukeEmResolver;

impl ActionResolver for MoveResolver {
    fn resolve(&self, entity: &Entity, manager: &EntityManager) -> Vec<Effect> {

        let resolver = Box::new(effect::MoveResolver {tile: self.tile.clone()});
        let effect = Effect {entity: entity.id.clone(), resolver: resolver};

        vec![effect]
    }
}

impl ActionResolver for AttackResolver {
    fn resolve(&self, entity: &Entity, manager: &EntityManager) -> Vec<Effect> {

        let target_entity = match self.target {
            Target::Entity(ref entity) => entity.clone(),
            _ => panic!("Only entity supported as target for now."),
        };

        let resolver = Box::new(effect::DamageResolver {damage: 2});
        let effect = Effect {entity: target_entity, resolver: resolver};

        vec![effect]
    }
}

impl ActionResolver for NukeEmResolver {
    fn resolve(&self, entity: &Entity, manager: &EntityManager) -> Vec<Effect> {

        let mut effects = vec![];

        for entity in manager.get_all() {
            let resolver = Box::new(effect::DamageResolver {damage: 999});
            let effect = Effect {entity: entity.id.clone(), resolver: resolver};
            effects.push(effect);
        }
        for entity in manager.get_all() {
            let resolver = Box::new(effect::RadiationResolver {radiation: 999});
            let effect = Effect {entity: entity.id.clone(), resolver: resolver};
            effects.push(effect);
        }

        effects
    }
}

pub enum Target {
    Entity(EntityId),
    Tile(Tile),
    None,
}
