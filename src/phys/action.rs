use phys::entity::Entity;

pub struct Action {
    pub entity_id: usize,
    pub resolve: Box<Fn (&mut Entity)>,
}
