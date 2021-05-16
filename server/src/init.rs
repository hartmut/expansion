// experimental initializations

use crate::core::entity::{player::Player, station::Station};
use amethyst::ecs::World;

pub fn init(world: &mut World) {
    let player = Player::new(world, "Luke Skywalker".to_string());
    let _station = Station::new(world, "ISS".to_string(), player);
}
