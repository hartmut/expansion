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
    fn new_player() {
        let one_player = Player::new("Ian Banks".to_string());
        let iam = Player::new("Ian Banks".to_string());
        let mut t: bool = false;
        if one_player.getuuid() == iam.getuuid() {
            t = true;
        };
        assert!(!t);
    }

}
