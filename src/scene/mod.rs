use creature::Creature;

pub struct Scene {
    name: &'static str,
    creatures: Vec<Creature>,
}
