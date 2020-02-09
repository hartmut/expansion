// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
use core::*;
use specs_hierarchy::HierarchySystem;

pub fn new<'a, 'b>(mut world: &mut specs::World) -> specs::Dispatcher<'a, 'b> {
    // register dispatcher
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(
            HierarchySystem::<component::partof::PartOf>::new(&mut world),
            "hierarchy_system",
            &[],
        )
        .build();

    dispatcher.setup(&mut world);
    dispatcher
}
