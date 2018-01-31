extern crate rand;
use rand::{Isaac64Rng, SeedableRng};
#[macro_use] extern crate ordermap;

// mod code;
mod play;
mod magic;
mod generate;
mod printing;
mod event_context;
mod buffs;

fn main() {
    // let combat_blink = magic::combat_blink();
    // println!("{:#?}", &combat_blink);
    // let mut rng = Isaac64Rng::from_seed(& vec![0,1,2,3]);
    let mut rng = rand::thread_rng();
    let spell = generate::spell(1, &mut rng);
    println!("{:#?}", &spell);
}
