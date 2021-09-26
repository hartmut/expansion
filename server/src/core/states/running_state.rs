// Standard State
use amethyst::{prelude::*, SimpleState, SimpleTrans};
use log::info;

pub struct RunningState;

impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        use crate::init::init;
        let StateData {
            world,
            resources: _,
            data: _,
        } = data;
        info!("loading world");
        // for tests
        init(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData>) -> SimpleTrans {
        amethyst::Trans::None
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData>) {
        // TODO save World
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData>) {}

    fn on_resume(&mut self, _data: StateData<'_, GameData>) {}

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData>,
        event: amethyst::StateEvent,
    ) -> amethyst::SimpleTrans {
        if let amethyst::StateEvent::Window(event) = &event {
            if amethyst::input::is_close_requested(event) {
                info!("saving world");
                amethyst::Trans::Quit
                // TODO save World
            } else {
                amethyst::Trans::None
            }
        } else {
            amethyst::Trans::None
        }
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans {
        Trans::None
    }

    fn shadow_fixed_update(&mut self, _data: StateData<'_, GameData>) {}

    fn shadow_update(&mut self, _data: StateData<'_, GameData>) {}
}
