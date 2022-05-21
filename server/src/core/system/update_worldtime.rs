use super::super::resource::worldtime::*;
use bevy::log::prelude::*;
use bevy::prelude::*;
use std::time::SystemTime;

pub fn update_worldtime(mut time: ResMut<Worldtime>) {
    //sleep and one more step
    time.tick_counter += 1;

    //update worldtime
    let time_now = SystemTime::now();
    time.step_leng = time_now
        .duration_since(time.time_last)
        .expect("SystemTime::duration_since failed");
    time.step_leng_warp = time.step_leng * (time.warp as u32);
    let chrono_step_leng = chrono::Duration::from_std(time.step_leng_warp).unwrap();
    time.worldtime = time.worldtime + chrono_step_leng;

    // save time of this step
    time.time_last = time_now;

    let secs = time.step_leng_warp.as_secs(); 
    info!("current time in worldtime {} and step_leng in worldtime is {secs}", time.worldtime);
}
