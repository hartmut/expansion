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
        // one world step every second
        let stepper = 1;
        app.add_systems(
            Update,
            (
                // update before other systems run
                update_worldtime::update_worldtime
                    .in_set(OneSecond::General),
                shadow_systems::shadow_clear
                    .in_set(OneSecond::ShadowClear)
                    .after(OneSecond::General),
                shadow_systems::shadow_update_module
                    .after(OneSecond::ShadowClear)
                    .in_set(OneSecond::ShadowUpdateModule),
                shadow_systems::shadow_update_station
                    .after(OneSecond::ShadowUpdateModule),
            )
            .run_if(on_timer(Duration::from_secs(stepper))),
        );

        // autosave every 2 seconds for resources and entities
        let savetimer = 2;
        app.add_systems(
            Update, (
            continous_save_resources.run_if(on_timer(Duration::from_secs(savetimer))),
            save_default()
            // TODO rewrite to include relevant resources in save of world
                .include_resource::<Worldtime>()
                .into_file("assets/saves/world.ron")
                .run_if(on_timer(Duration::from_secs(savetimer))),
            )
        );

    }
}
