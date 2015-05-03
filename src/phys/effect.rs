use phys::area::Tile;
use phys::entity::Entity;
use phys::entity::EntityId;

pub struct Effect {
    pub entity: EntityId,
    pub resolver: Box<EffectResolver>,
}

impl Effect {
    pub fn resolve(&self, entity: &mut Entity) {
        self.resolver.resolve(entity);
    }
}

pub trait EffectResolver {
    fn resolve(&self, &mut Entity);
}

pub struct MoveResolver {
    pub tile: Tile,
}

pub struct DamageResolver {
    pub damage: i32,
}

impl EffectResolver for MoveResolver {
    fn resolve(&self, entity: &mut Entity) {
        entity.tile = self.tile.clone();
    }
}

impl EffectResolver for DamageResolver {
    fn resolve(&self, entity: &mut Entity) {
        entity.damage += self.damage;
    }
}
