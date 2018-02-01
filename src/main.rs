extern crate rand;
extern crate piston_window;
extern crate find_folder;

mod play;
mod magic;
mod generate;
mod printing;
mod event_context;
mod buffs;
mod movement_2d;
mod wasd_set;

fn main() {
    play::game_loop();
}
