pub mod continous_save;
pub mod entity_save;
pub mod load_scene;
pub mod shadow_systems;
pub mod update_worldtime;
use bevy::{prelude::*, time::common_conditions::*, utils::Duration};

use self::continous_save::continous_save;

pub struct ExpSystems;

#[derive(SystemSet, Hash, Eq, Debug, Clone, PartialEq)]
enum OneSecond {
    ShadowClear,
    ShadowUpdateModule,
}

impl Plugin for ExpSystems {
    fn build(&self, app: &mut App) {
        // insert systems for step updates

        // one world step every second
        // COMEBACK complete rewrite of system especially labels
        // TODO rewrite labels with SystemSet https://docs.rs/bevy/latest/bevy/ecs/schedule/trait.FreeSystemSet.html
        // because of the new scheduler - better use on_fixed_timer? or is there a possibility for time controll
        // for a whole system set?
        app.add_systems(
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
        ));

        // autosave every x seconds
        app.add_system(
            //     // .with_system(entity_save::entity_save.exclusive_system())
        // TODO at_end() for contious save necessary? and entitysave needs got get implemtend
            continous_save.run_if(on_timer(Duration::from_secs(5))),
        );
    }
}
