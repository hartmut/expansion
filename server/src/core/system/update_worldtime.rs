use super::super::resource::worldtime::*;
use bevy::prelude::*;
use hifitime::Duration;
use std::time::SystemTime;

pub fn update_worldtime(mut time: ResMut<Worldtime>) {
    //step up the tick_counter
    time.tick_counter += 1;

    // get current Systemtime
    let time_now = SystemTime::now();

    // get length of time step since last update of Worldtime Struct
    time.step_leng = time_now
        .duration_since(time.time_last)
        .expect("SystemTime::duration_since failed");

    // multiply with warp time
    time.step_leng_warp = time.step_leng * (time.warp as u32);

    // convert warped step leng to seconds
    let secs = time.step_leng_warp.as_secs_f64();

    // create Duraction Struct for addition to epoch
    let delta = Duration::from_seconds(secs);

    // add delta time to the epoch variable - current time in simulation is now in epoch
    time.epoch += delta;

    // debug information
    // info!("current epoch time in worldtime {}", time.epoch);

    // save time of this step in time_last for next iteration
    time.time_last = time_now;
}
