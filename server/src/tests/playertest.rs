// Experimental Simulator of a cooperative solar system economy.
// Copyright (C) 2016  Hartmut Prochaska
// See doc/LICENSE for licensing information
//
//


#[cfg(test)]
mod tests {
    use common::stdtrait::StdTrait;
    use character::player::Player;

    #[test]
    fn do_i_realy_get_a_new_uuid_for_a_new_player() {
        let one_player = Player::new("Ian Banks".to_string());
        let iam = Player::new("Ian Banks".to_string());
        assert!(!(one_player.getuuid() == iam.getuuid()));
    }

}
