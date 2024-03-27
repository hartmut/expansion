use super::super::resource::worldtime::*;
use bevy::prelude::*;
use hifitime::Duration;

pub fn update_worldtime(mut worldtime: ResMut<Worldtime>, time: Res<Time<Real>>) {
    //step up the tick_counter
    worldtime.tick_counter += 1;

    // get length of time step since last update of Worldtime Struct
    let step_leng = time.elapsed() - worldtime.lasttime;

    // multiply with warp time
    worldtime.step_leng_warp = step_leng * worldtime.warp;

    // convert warped step leng to seconds
    let secs = worldtime.step_leng_warp.as_secs_f64();

    // create Duraction Struct for addition to epoch
    let delta = Duration::from_seconds(secs);

    // add delta time to the epoch variable - current time in simulation is now in epoch
    worldtime.epoch += delta;

    // debug information
    // info!("current epoch time in worldtime {}", worldtime.epoch);

    // update lasttime
    worldtime.lasttime = time.elapsed();
}
