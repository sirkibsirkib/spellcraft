use code2::*;
use rand::{Rng};


pub fn spell<R: Rng>(max_depth: u16, rng: &mut R) -> Spell {
    assert!(max_depth > 0);
    let slots = SlotsTaken {ent:1,ent_set:0,loc:0,disc:0};
    Spell {
        on_cast: vec_instruction(rng, max_depth as i16-1, &mut slots.clone()),
        requires: condition(rng, max_depth as i16-1, &mut slots.clone()),
        consumes: vec_resource(rng, max_depth as i16-1, &mut slots),
    }
}

#[derive(Clone)]
struct SlotsTaken {
    ent: u8,
    ent_set: u8,
    loc: u8,
    disc: u8,
}

pub fn vec_instruction<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Instruction> {
    let mut v = vec![];
    while rng.gen_weighted_bool(3) {
        v.push(instruction(rng, depth_left-1, slots));
    }
    v
}

pub fn condition<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Condition {
    let stop = rng.gen_weighted_bool(depth_left as u32 + 1);
    if stop {
        if rng.gen::<bool>() {
            Condition::Top
        } else {
            Condition::Bottom
        }
    } else {
        match rng.gen::<u8>() % 70 {
            n if n < 10 => Condition::Nand(vec_condition(rng, depth_left-1, slots)),
            n if n < 25 => Condition::And(vec_condition(rng, depth_left-1, slots)),
            n if n < 45 => Condition::Or(vec_condition(rng, depth_left-1, slots)),
            n if n < 55 => Condition::Equals(
                discrete(rng, depth_left-1, slots),
                discrete(rng, depth_left-1, slots),
            ),
            n if n < 60 => Condition::LessThan(
                discrete(rng, depth_left-1, slots),
                discrete(rng, depth_left-1, slots),
            ),
            n if n < 65 => Condition::MoreThan(
                discrete(rng, depth_left-1, slots),
                discrete(rng, depth_left-1, slots),
            ),
            _ => Condition::EntitySetCmp(entity_set_cmp(rng, depth_left-1, slots)),
        }
    }
}

pub fn discrete<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Discrete {
    let stop = rng.gen_weighted_bool(depth_left as u32 + 1);
    use code2::Discrete::*;
    if stop {
        match rng.gen::<u8>() % 35 {
            x if x < 20 && slots.disc > 0 => {
                LoadFrom(DSlot(rng.gen::<u8>() % slots.disc))
            },
            x if x < 20 => Const(rng.gen::<i32>() % 50),
            x if x < 27 => {
                let a = rng.gen::<i32>() % 50;
                let b = rng.gen::<i32>() % 50;
                if a < b {Range(a,b)}
                else {Range(b,a)}
            },
            _ => WithinPercent(rng.gen::<i32>() % 50, rng.gen::<f32>()),
        }
    } else {
        match rng.gen::<u8>() % 53 {
            x if x < 5 => Div(
                Box::new(discrete(rng, depth_left-1, slots)),
                Box::new(discrete(rng, depth_left-1, slots)),
            ),
            x if x < 20 => Sum(vec_discrete(rng, depth_left-1, slots)),
            x if x < 28 => Neg(Box::new(discrete(rng, depth_left-1, slots))),
            x if x < 32 => Mult(vec_discrete(rng, depth_left-1, slots)),
            x if x < 38 => Max(vec_discrete(rng, depth_left-1, slots)),
            x if x < 44 => Min(vec_discrete(rng, depth_left-1, slots)),
            x if x < 48 => CountStacks(buff(rng), entity(rng, depth_left-1, slots)),
            x if x < 48 => CountDur(buff(rng), entity(rng, depth_left-1, slots)),
            x if x < 51 => Choose(vec_discrete(rng, depth_left-1, slots)),
            _ => Cardinality(Box::new(entity_set(rng, depth_left, slots))),
        }
    }
}

pub fn vec_discrete<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Discrete> {
    if depth_left == 0 { return vec![] }
    let mut v = vec![];
    while rng.gen_weighted_bool(3) {
        v.push(discrete(rng, depth_left-1, slots));
    }
    v
}

pub fn vec_condition<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Condition> {
    if depth_left == 0 { return vec![] }
    let mut v = vec![];
    while rng.gen_weighted_bool(3) {
        v.push(condition(rng, depth_left-1, slots));
    }
    v
}

pub fn entity<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Entity {
    let stop = rng.gen_weighted_bool(depth_left as u32 + 1);
    use code2::Entity::*;
    if stop && slots.ent > 0 {
        LoadEntity(ESlot(rng.gen::<u8>() % slots.ent))
    } else {
        match rng.gen::<u8>() % 29 {
            x if x < 10 => FirstOf(Box::new(entity_set(rng, depth_left-1, slots))),
            x if x < 15 => Choose(Box::new(entity_set(rng, depth_left-1, slots))),
            x if x < 25 => ClosestFrom(
                Box::new(entity_set(rng, depth_left-1, slots)),
                Box::new(location(rng, depth_left-1, slots)),
            ),
            _ => LastOf(Box::new(entity_set(rng, depth_left-1, slots))),
        }
    }
}

pub fn location<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Location {
    use code2::Location::*;
    let stop = rng.gen_weighted_bool(depth_left as u32 + 1);
    if stop && slots.loc > 0 {
        LoadLocation(LSlot(rng.gen::<u8>() % slots.loc))
    } else {
        match rng.gen::<u8>() % 20 {
            x if x < 15 => AtEntity(entity(rng, depth_left-1, slots)),
            x if x < 17 => Midpoint(vec_location(rng, depth_left-1, slots)),
            _ => Choose(vec_location(rng, depth_left-1, slots)),
        }
    }
}

pub fn entity_set_cmp<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> EntitySetCmp {

}

pub fn entity_set<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> EntitySet {

}

pub fn instruction<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Instruction {

}

pub fn vec_location<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Location> {
    let mut v = vec![];
    v.push(location(rng, depth_left-1, slots));
    while rng.gen_weighted_bool(3) {
        v.push(location(rng, depth_left-1, slots));
    }
    v
}

pub fn vec_resource<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Resource> {
    let mut v = vec![];
    v.push(resource(rng, depth_left-1, slots));
    while rng.gen_weighted_bool(3) {
        v.push(resource(rng, depth_left-1, slots));
    }
    v
}

pub fn resource<R: Rng>(rng: &mut R, depth_left: i16, slots: &mut SlotsTaken) -> Resource {
    use code2::Resource::*;
    match rng.gen::<u8>() % 24 {
        x if x < 10 => Mana(discrete(rng, depth_left-1, slots)),
        x if x < 17 => Health(discrete(rng, depth_left-1, slots)),
        _ => BuffStacks(
            buff(rng),
            discrete(rng, depth_left-1, slots),
        ),
    }
}

pub fn buff<R: Rng>(rng: &mut R) -> Buff {
    use code2::Buff::*;
    match rng.gen::<u8>() % 50 {
        x if x < 5 => Swarm,
        x if x < 15 => Burned,
        x if x < 25 => Cold,
        x if x < 35 => Chilled,
        x if x < 40 => Toxified,
        x if x < 43 => Envenomed,
        x if x < 50 => Electrified,
    }
}

