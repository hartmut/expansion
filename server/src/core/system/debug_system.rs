use amethyst::{core::{ecs::System, transform::Children}, prelude::*};
use log::info;

pub struct DebugSystem;

impl System for DebugSystem {
    fn build(self) -> Box<dyn ParallelRunnable> {
        Box::new(
            SystemBuilder::new("DebugSystem")
                .with_query(<&mut Children>::query())
                .read_component::<Children>()
                .build(move |_commands, world, _time, query| {
                    for x in query.iter_mut(world) {
                        info!("Debug Info {:?}", x);
                    }
                }),
        )
    }
}
