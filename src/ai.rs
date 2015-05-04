use phys::action;
use phys::action::Action;
use phys::area::Tile;
use phys::entity::Entity;

pub fn decide(entity: &Entity) -> Action {

    //get_move_action(entity)
    //get_attack_action(entity)
    get_nuke_em_action(entity)
}

#[allow(dead_code)]
fn get_move_action(entity: &Entity) -> Action {

    let Tile(x, y) = entity.tile;
    let tile = Tile(x + 1, y + 1);

    let resolver = Box::new(action::MoveResolver {tile: tile});
    let action = Action {entity_id: entity.id.clone(), resolver: resolver};

    action
}

#[allow(dead_code)]
fn get_attack_action(entity: &Entity) -> Action {

    let target = action::Target::Entity(entity.id.clone()); // suicidal

    let resolver = Box::new(action::AttackResolver {target: target});
    let action = Action {entity_id: entity.id.clone(), resolver: resolver};

    action
}

fn get_nuke_em_action(entity: &Entity) -> Action {

    let resolver = Box::new(action::NukeEmResolver);
    let action = Action {entity_id: entity.id.clone(), resolver: resolver};

    action
}