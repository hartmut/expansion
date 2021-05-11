use amethyst::prelude::*;
use core::component::desc::Desc;

pub struct Module;

impl Module {
    pub fn new(world: &mut World, name: String, _parent: Entity) -> Entity {
        let desc = Desc::new(name, "".to_string());
        world.push((desc,))
    }
}
