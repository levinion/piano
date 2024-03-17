mod note;
mod player;

use std::fs::read_to_string;

use player::Player;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut player = Player::new().set_speed(2.0);
    let input = read_to_string(&args[1]).unwrap();
    player.add_notes(&input);
    player.write(&args[2]);
}
