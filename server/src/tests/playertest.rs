// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//

use character::player::Player;

#[test]
fn create_player() {
    let iam = Player::new("Hartmut".to_string());
}
