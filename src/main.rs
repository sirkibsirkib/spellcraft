extern crate rand;

// mod code;
// mod play;
mod code2;
mod generate;

fn main() {
    let combat_blink = code2::combat_blink();
    println!("{:#?}", &combat_blink);
}
