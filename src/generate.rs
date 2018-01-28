use code2::*;
use rand::{Rng};


pub fn spell<R: Rng>(max_depth: u8, rng: &mut R) -> Spell {
    assert!(max_depth > 0);
    let slots = SlotsTaken {ent:1,ent_set:0,loc:0,disc:0};
    Spell {
        on_cast: vec_instruction(rng, max_depth-1, slots.clone()),
        requires: condition(rng, max_depth-1, slots),
        consumes: vec_resource
    }
}

#[derive(Clone)]
struct SlotsTaken {
    ent: u8,
    ent_set: u8,
    loc: u8,
    disc: u8,
}

pub fn vec_instruction<R: Rng>(rng: &mut R, depth_left: u8, mut slots: SlotsTaken) -> Vec<Instruction> {
    let mut v = vec![];

    v
}

pub fn instruction<R: Rng>(rng: &mut R, depth_left: u8, mut slots: SlotsTaken) -> Instruction {
    
}

pub fn vec_resource<R: Rng>(rng: &mut R, depth_left: u8, mut slots: SlotsTaken) -> Vec<Resource> {

}