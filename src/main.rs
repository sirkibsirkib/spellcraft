extern crate rand;
use rand::{Isaac64Rng, SeedableRng};


// mod code;
mod play;
mod magic;
mod generate;
mod printing;
mod buffs;

fn main() {
    // let combat_blink = magic::combat_blink();
    // println!("{:#?}", &combat_blink);
    // let mut rng = Isaac64Rng::from_seed(& vec![0,1,2,3]);
    let mut rng = Isaac64Rng::new_unseeded();
    for _ in 0..10 {
        println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~\n{:#?}", &generate::spell(1, &mut rng));
    }
}
