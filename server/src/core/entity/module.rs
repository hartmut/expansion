use crate::core::component::desc::Desc;
use crate::core::component::basics::BasicParameter;
use bevy::ecs::system::EntityCommands;
use bevy_inspector_egui::Inspectable;
use bevy::prelude::*;

#[derive(Bundle, Reflect, Inspectable)]
pub struct Module{
    desc: Desc,
    basics: BasicParameter,
}

// impl Module {
//     pub fn create(world: &mut World, name: String, _parent: Entity) -> Entity {
//         let desc = Desc::new(name, "");
//         world.push((desc, Parent))
//     }
// }
impl Module {
    pub fn create(
        mut commands: EntityCommands,
        name: impl Into<String>,
    ) -> Entity {
        let desc = Desc::new(name, "");
        let extend = Vec3::new(5.0,3.0,3.0);
        let basics = BasicParameter{
             mass: 1000.0,
             usablevol: 5.0,
             outervol: 6.0,
             extend,
        };
        commands.insert_bundle(Module { desc, basics }).id()
    }
}
