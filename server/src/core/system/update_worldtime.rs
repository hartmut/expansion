use amethyst::{core::ecs::System, prelude::*};
use std::time::SystemTime;

use super::super::resource::worldtime::*;
use bevy::log::prelude::*;
use bevy::prelude::*;

pub struct UpdateWorldtime;

impl System for UpdateWorldtime {
    fn build(self) -> Box<dyn ParallelRunnable> {
        Box::new(
            SystemBuilder::new("UpdateWorldtime")
                .write_resource::<Worldtime>()
                .build(move |_commands, _world, time, _query| {
                    //sleep and one more step
                    time.tick_counter += 1;

                    //update worldtime
                    let time_now = SystemTime::now();
                    time.step_leng = time_now
                        .duration_since(time.time_last)
                        .expect("SystemTime::duration_since failed");
                    let step_leng_warp = time.step_leng * (time.warp as u32);
                    let chrono_step_leng = chrono::Duration::from_std(step_leng_warp).unwrap();
                    time.worldtime = time.worldtime + chrono_step_leng;

                    // save time of this step
                    time.time_last = time_now;

                    info!("step into worldtime {}", time.worldtime);
                }),
        )
    }
}

pub fn update_worldtime(mut time: ResMut<Worldtime>) {
    //sleep and one more step
    time.tick_counter += 1;

    //update worldtime
    let time_now = SystemTime::now();
    time.step_leng = time_now
        .duration_since(time.time_last)
        .expect("SystemTime::duration_since failed");
    let step_leng_warp = time.step_leng * (time.warp as u32);
    let chrono_step_leng = chrono::Duration::from_std(step_leng_warp).unwrap();
    time.worldtime = time.worldtime + chrono_step_leng;

    // save time of this step
    time.time_last = time_now;

    info!("step into worldtime {}", time.worldtime);
}
