use phys::area::Tile;
use phys::entity::Entity;

pub struct Effect {
    pub entity_id: i64,
    pub resolver: Box<EffectResolver>,
}

pub trait EffectResolver {
    fn resolve(&self, &mut Entity);
}

pub struct DamageResolver {
    pub damage: i64,
}

pub struct MovementResolver {
    pub tile: Tile,
}

pub struct RadiationResolver {
    pub radiation: i64,
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
