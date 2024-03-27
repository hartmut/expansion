pub mod continous_save;
pub mod entity_save;
pub mod load_scene;
pub mod shadow_systems;
pub mod update_worldtime;
use self::continous_save::continous_save_resources;
use crate::core::resource::worldtime::*;
use bevy::{prelude::*, time::common_conditions::*, utils::Duration};
use moonshine_save::prelude::*;

pub struct ExpSystems;

#[derive(SystemSet, Hash, Eq, Debug, Clone, PartialEq)]
enum OneSecond {
    ShadowClear,
    ShadowUpdateModule,
    General,
}

impl Plugin for ExpSystems {
    fn build(&self, app: &mut App) {
        // TODO insert systems for step updates

        // one world step every second
        // NOTE complete rewrite of system especially labels
        // because of the new scheduler - better use on_fixed_timer? or is there a possibility for time controll
        // for a whole system set?
        app.add_systems(
            Update,
            (
                shadow_systems::shadow_clear
                    .in_set(OneSecond::ShadowClear)
                    .run_if(on_timer(Duration::from_secs(1))),
                shadow_systems::shadow_update_module
                    .after(OneSecond::ShadowClear)
                    .run_if(on_timer(Duration::from_secs(1)))
                    .in_set(OneSecond::ShadowUpdateModule),
                shadow_systems::shadow_update_station
                    .after(OneSecond::ShadowUpdateModule)
                    .run_if(on_timer(Duration::from_secs(1))),
                // update before other systems run
                update_worldtime::update_worldtime
                    .in_set(OneSecond::General)
                    .run_if(on_timer(Duration::from_secs(1))),
            ),
        );

        // autosave every 2 seconds for resources and entities
        app.add_systems(
            Update, (
            continous_save_resources.run_if(on_timer(Duration::from_secs(2))),
            save_default()
            // TODO rewrite to include some resources in save of world
                .include_resource::<Worldtime>()
                .into_file("assets/saves/world.ron")
                .run_if(on_timer(Duration::from_secs(2))),
            )
        );

    }
}
