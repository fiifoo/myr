use phys::area::Tile;
use phys::entity::Entity;
use phys::entity::EntityId;

pub struct Effect {
    pub entity_id: EntityId,
    pub resolver: Box<EffectResolver>,
}

pub trait EffectResolver {
    fn resolve(&self, &mut Entity);
}

pub struct DamageResolver {
    pub damage: i32,
}

pub struct MovementResolver {
    pub tile: Tile,
}

pub struct RadiationResolver {
    pub radiation: i32,
}

impl Effect {
    pub fn resolve(&self, entity: &mut Entity) {
        self.resolver.resolve(entity);
    }
}

impl EffectResolver for DamageResolver {
    fn resolve(&self, entity: &mut Entity) {
        entity.damage += self.damage;
    }
}

impl EffectResolver for MovementResolver {
    fn resolve(&self, entity: &mut Entity) {
        entity.tile = self.tile.clone();
    }
}

impl EffectResolver for RadiationResolver {
    fn resolve(&self, entity: &mut Entity) {
        entity.radiation += self.radiation;
    }
}
