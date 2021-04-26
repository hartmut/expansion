// Standard State
use amethyst::{prelude::*, GameData, SimpleState, SimpleTrans, StateData};

pub struct RunningState;

impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        // COMBAK continue, integrate time and bundle resources
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        amethyst::Trans::None
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: amethyst::StateEvent,
    ) -> amethyst::SimpleTrans {
        if let amethyst::StateEvent::Window(event) = &event {
            if amethyst::input::is_close_requested(&event) {
                amethyst::Trans::Quit
            } else {
                amethyst::Trans::None
            }
        } else {
            amethyst::Trans::None
        }
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }

    fn shadow_fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}

    fn shadow_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
