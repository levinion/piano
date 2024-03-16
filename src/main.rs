mod note;
mod player;

use std::fs::read_to_string;

use player::Player;

fn main() {
    let input: Vec<_> = std::env::args().collect();
    let player = Player::new().set_speed(2.0);
    let input = read_to_string(&input[1]).unwrap();
    player.play(&input);
}
