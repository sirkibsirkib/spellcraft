use magic::*;
use rand::{Rng};
use std::collections::HashMap;


pub fn spell<R: Rng>(max_depth: u16, rng: &mut R) -> (Spell, u32) {
    assert!(max_depth > 0);
    let mut slots = SlotsTaken {ent:1,ent_set:0,loc:0,disc:0};
    let mut counter = Counter{ counter: 0 };
    let s = Spell {
        on_cast: vec_instruction(rng, &mut counter,  max_depth as i16-1, &mut slots.clone()),
        requires: condition(rng, &mut counter,  max_depth as i16-1, &mut slots.clone()),
        consumes: vec_resource(rng, &mut counter,  max_depth as i16-1, &mut slots),
    };
    (s, counter.counter)
}

struct Counter {
    counter: u32,
}
impl Counter {
    pub fn increment(&mut self) {
        self.counter += 1;
    }
}

#[derive(Clone, Copy)]
struct SlotsTaken {
    ent: u8,
    ent_set: u8,
    loc: u8,
    disc: u8,
}

fn vec_instruction<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Instruction> {
    counter.increment();
    let mut v = vec![];
    let stored = slots.clone();
    v.push(instruction(rng, counter,  depth_left-1, slots));
    while rng.gen_weighted_bool(3) {
        v.push(instruction(rng, counter,  depth_left-1, slots));
    }
    *slots = stored; // replace. local vars get OUTTA HEE
    v
}

fn condition<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Condition {
    counter.increment();
    let stop = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    if stop {
        if rng.gen::<bool>() {
            Condition::Top
        } else {
            Condition::Bottom
        }
    } else {
        match rng.gen::<u8>() % 70 {
            n if n < 10 => Condition::Nand(vec_condition(rng, counter,  depth_left-1, slots)),
            n if n < 25 => Condition::And(vec_condition(rng, counter,  depth_left-1, slots)),
            n if n < 45 => Condition::Or(vec_condition(rng, counter,  depth_left-1, slots)),
            n if n < 55 => Condition::Equals(
                discrete(rng, counter,  depth_left-1, slots),
                discrete(rng, counter,  depth_left-1, slots),
            ),
            n if n < 60 => Condition::LessThan(
                discrete(rng, counter,  depth_left-1, slots),
                discrete(rng, counter,  depth_left-1, slots),
            ),
            n if n < 65 => Condition::MoreThan(
                discrete(rng, counter,  depth_left-1, slots),
                discrete(rng, counter,  depth_left-1, slots),
            ),
            _ => Condition::EntitySetCmp(entity_set_cmp(rng, counter,  depth_left-1, slots)),
        }
    }
}

fn discrete<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Discrete {
    counter.increment();
    let stop = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    use magic::Discrete::*;
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
                Box::new(discrete(rng, counter,  depth_left-1, slots)),
                Box::new(discrete(rng, counter,  depth_left-1, slots)),
            ),
            x if x < 20 => Sum(vec_discrete(rng, counter,  depth_left-1, slots)),
            x if x < 28 => Neg(Box::new(discrete(rng, counter,  depth_left-1, slots))),
            x if x < 32 => Mult(vec_discrete(rng, counter,  depth_left-1, slots)),
            x if x < 38 => Max(vec_discrete(rng, counter,  depth_left-1, slots)),
            x if x < 44 => Min(vec_discrete(rng, counter,  depth_left-1, slots)),
            x if x < 48 => CountStacks(buff(rng, counter), entity(rng, counter,  depth_left-1, slots)),
            x if x < 48 => CountDur(buff(rng, counter), entity(rng, counter,  depth_left-1, slots)),
            x if x < 51 => Choose(vec_discrete(rng, counter,  depth_left-1, slots)),
            _ => Cardinality(Box::new(entity_set(rng, counter,  depth_left, slots))),
        }
    }
}

fn vec_discrete<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Discrete> {
    counter.increment();
    if depth_left == 0 { return vec![] }
    let mut v = vec![];
    while rng.gen_weighted_bool(3) {
        v.push(discrete(rng, counter,  depth_left-1, slots));
    }
    v
}

fn vec_condition<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Condition> {
    counter.increment();
    if depth_left == 0 { return vec![] }
    let mut v = vec![];
    while rng.gen_weighted_bool(2) {
        v.push(condition(rng, counter,  depth_left-1, slots));
    }
    v
}

fn entity<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Entity {
    counter.increment();
    let stop = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    use magic::Entity::*;
    if stop && slots.ent > 0 {
        LoadEntity(ESlot(rng.gen::<u8>() % slots.ent))
    } else {
        match rng.gen::<u8>() % 29 {
            x if x < 10 => FirstOf(Box::new(entity_set(rng, counter,  depth_left-1, slots))),
            x if x < 15 => Choose(Box::new(entity_set(rng, counter,  depth_left-1, slots))),
            x if x < 25 => ClosestFrom(
                Box::new(entity_set(rng, counter,  depth_left-1, slots)),
                Box::new(location(rng, counter,  depth_left-1, slots)),
            ),
            _ => LastOf(Box::new(entity_set(rng, counter,  depth_left-1, slots))),
        }
    }
}

fn location<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Location {
    counter.increment();
    use magic::Location::*;
    let stop = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    if stop && slots.loc > 0 {
        LoadLocation(LSlot(rng.gen::<u8>() % slots.loc))
    } else {
        match rng.gen::<u8>() % 20 {
            x if x < 15 => AtEntity(entity(rng, counter,  depth_left-1, slots)),
            x if x < 17 => Midpoint(vec_location(rng, counter,  depth_left-1, slots)),
            _ => Choose(vec_location(rng, counter,  depth_left-1, slots)),
        }
    }
}

fn entity_set_cmp<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> EntitySetCmp {
    counter.increment();
    use magic::EntitySetCmp::*;
    let stoppish = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    if stoppish && slots.loc > 0 {
        match rng.gen::<u8>() % 20 {
            x if x < 10 => Contains(
                entity_set(rng, counter,  depth_left-1, slots),
                entity(rng, counter,  depth_left-1, slots),
            ),
            x if x < 13 => Subset(
                entity_set(rng, counter,  depth_left-1, slots),
                entity_set(rng, counter,  depth_left-1, slots),
            ),
            x if x < 16 => Superset(
                entity_set(rng, counter,  depth_left-1, slots),
                entity_set(rng, counter,  depth_left-1, slots),
            ),
            _ => Equal(
                entity_set(rng, counter,  depth_left-1, slots),
                entity_set(rng, counter,  depth_left-1, slots),
            ),
        }
    } else {
        match rng.gen::<u8>() % 17 {
            x if x < 2 => Nand(vec_entity_set_cmp(rng, counter,  depth_left-1, slots)),
            x if x < 10 => And(vec_entity_set_cmp(rng, counter,  depth_left-1, slots)),
            _ => Or(vec_entity_set_cmp(rng, counter,  depth_left-1, slots)),
        }
    }
}

fn vec_entity_set<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Vec<EntitySet> {
    counter.increment();
    let mut v = vec![];
    v.push(entity_set(rng, counter,  depth_left-1, slots));
    while rng.gen_weighted_bool(3) {
        v.push(entity_set(rng, counter,  depth_left-1, slots));
    }
    v
}

fn vec_entity_set_cmp<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Vec<EntitySetCmp> {
    counter.increment();
    let mut v = vec![];
    v.push(entity_set_cmp(rng, counter,  depth_left-1, slots));
    while rng.gen_weighted_bool(3) {
        v.push(entity_set_cmp(rng, counter,  depth_left-1, slots));
    }
    v
}

fn vec_location<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Location> {
    counter.increment();
    let mut v = vec![];
    v.push(location(rng, counter,  depth_left-1, slots));
    while rng.gen_weighted_bool(3) {
        v.push(location(rng, counter,  depth_left-1, slots));
    }
    v
}

fn vec_resource<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Resource> {
    counter.increment();
    let mut v = vec![];
    v.push(resource(rng, counter,  depth_left-1, slots));
    while rng.gen_weighted_bool(3) {
        v.push(resource(rng, counter,  depth_left-1, slots));
    }
    v
}

fn resource<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Resource {
    counter.increment();
    use magic::Resource::*;
    match rng.gen::<u8>() % 24 {
        x if x < 10 => Mana(discrete(rng, counter,  depth_left-1, slots)),
        x if x < 17 => Health(discrete(rng, counter,  depth_left-1, slots)),
        _ => BuffStacks(
            buff(rng, counter),
            discrete(rng, counter,  depth_left-1, slots),
        ),
    }
}

fn buff<R: Rng>(rng: &mut R, counter: &mut Counter) -> Buff {
    counter.increment();
    use magic::Buff::*;
    match rng.gen::<u8>() % 50 {
        x if x < 5 => Swarm,
        x if x < 15 => Burned,
        x if x < 25 => Cold,
        x if x < 35 => Chilled,
        x if x < 40 => Toxified,
        x if x < 43 => Envenomed,
        _ => Electrified,
    }
}




fn entity_set<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> EntitySet {
    counter.increment();
    use magic::EntitySet::*;
    let stop = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    if stop {
        match rng.gen::<u8>() % 25 {
            x if x < 15 && slots.loc > 0 => {
                IsInSlot(ESetSlot(rng.gen::<u8>() % slots.loc))
            },
            x if x < 10 => IsHuman,            
            x if x < 15 => IsProjectile,            
            x if x < 20 => Universe,
            _ => Empty,
        }
    } else {
        match rng.gen::<u8>() % 35 {
            x if x < 2 => Nand(vec_entity_set(rng, counter,  depth_left-1, slots)),
            x if x < 10 => And(vec_entity_set(rng, counter,  depth_left-1, slots)),
            x if x < 15 => Or(vec_entity_set(rng, counter,  depth_left-1, slots)),
            x if x < 18 => Only(entity(rng, counter,  depth_left-1, slots)),
            x if x < 23 => WithinRangeOf(
                entity(rng, counter,  depth_left-1, slots),
                discrete(rng, counter,  depth_left-1, slots),
            ),
            x if x < 27 => HasMinResource(
                entity(rng, counter,  depth_left-1, slots),
                resource(rng, counter,  depth_left-1, slots)
            ),
            x if x < 32 => EnemiesOf(entity(rng, counter,  depth_left-1, slots)),
            _ => AllBut(entity(rng, counter,  depth_left-1, slots)),
        }
    }
}

fn vec_direction<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Vec<Direction> {
    counter.increment();
    let mut v = vec![];
    v.push(direction(rng, counter,  depth_left-1, slots));
    while rng.gen_weighted_bool(3) {
        v.push(direction(rng, counter,  depth_left-1, slots));
    }
    v
}

fn direction<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Direction {
    counter.increment();
    use magic::Direction::*;
    let stop = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    if stop {
        if rng.gen() {
            ConstRad(rng.gen::<f32>() * 3.0 - 1.5)
        } else {
            let a = rng.gen::<f32>() * 3.0 - 1.5;
            let b = rng.gen::<f32>() * 3.0 - 1.5;
            if a < b {
                BetweenRad(a,b)
            } else {
                BetweenRad(b,a)
            }
        }
    } else {
        if rng.gen() {
            Choose(vec_direction(rng, counter,  depth_left-1, slots))
        } else {
            ChooseWithinRadOf(
                Box::new(direction(rng, counter,  depth_left-1, slots)),
                rng.gen::<f32>() * 3.0 - 1.5,
            )
        }
    }
}

fn definition<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Definition {
    counter.increment();
    use magic::Definition::*;
    match rng.gen::<u8>() % 10 {
        x if x < 3 => {
            let y = entity_set(rng, counter,  depth_left-1, slots);
            slots.ent_set += 1;
            ESet(
                ESetSlot(slots.ent_set - 1),
                y,
            )
        },
        x if x < 7 => {
            let y = entity(rng, counter,  depth_left-1, slots);
            slots.ent += 1;
            E(
                ESlot(slots.ent - 1),
                y,
            )
        },
        _ => {
            let y = discrete(rng, counter,  depth_left-1, slots);
            slots.disc += 1;
            D(
                DSlot(slots.disc - 1),
                y,
            )
        },
    }
}

fn projectile_blueprint<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16) -> ProjectileBlueprint {
    counter.increment();
    let just_me = SlotsTaken {ent:1,ent_set:0,loc:0,disc:0};
    ProjectileBlueprint {
        on_create: instruction(rng, counter,  depth_left-1, &mut just_me.clone()),
        on_collision: instruction(rng, counter,  depth_left-1, &mut SlotsTaken {ent:2,ent_set:0,loc:0,disc:0}),
        collides_with: entity_set(rng, counter,  depth_left-1, &mut just_me.clone()),
        on_destroy: instruction(rng, counter,  depth_left-1, &mut just_me.clone()),
        lifetime: discrete(rng, counter,  depth_left-1, &mut just_me.clone()),
    }
}


fn instruction<R: Rng>(rng: &mut R, counter: &mut Counter, depth_left: i16, slots: &mut SlotsTaken) -> Instruction {
    counter.increment();
    use magic::Instruction::*;
    let stop1 = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    let stop2 = depth_left <= 1 || rng.gen_weighted_bool(depth_left as u32 + 1);
    if stop1 || stop2 {
        match rng.gen::<u8>() % 40 {
            x if x < 10 => Define(definition(rng, counter,  depth_left-1, slots)),
            x if x < 13 => DestroyWithoutEvent(entity(rng, counter,  depth_left-1, slots)),
            x if x < 18 => Destroy(entity(rng, counter,  depth_left-1, slots)),
            x if x < 22 => MoveEntity(
                entity(rng, counter,  depth_left-1, slots),
                location(rng, counter,  depth_left-1, slots),
            ),
            x if x < 30 => AddResource(
                entity(rng, counter,  depth_left-1, slots),
                resource(rng, counter,  depth_left-1, slots),
            ),
            x if x < 34 => AddVelocity(
                entity(rng, counter,  depth_left-1, slots),
                direction(rng, counter,  depth_left-1, slots),
                discrete(rng, counter,  depth_left-1, slots),
            ),
            _ => SpawnProjectileAt(
                Box::new(projectile_blueprint(rng, counter,  depth_left-1)),
                location(rng, counter,  depth_left-1, slots),
            )
        }
    } else {
        match rng.gen::<u8>() % 11 {
            x if x < 4 => ITE(
                condition(rng, counter,  depth_left-1, slots),
                vec_instruction(rng, counter,  depth_left-1, slots),
                vec_instruction(rng, counter,  depth_left-1, slots),
            ),
            x if x < 6 => CallWith(
                definition(rng, counter,  depth_left-1, slots),
                vec_instruction(rng, counter,  depth_left-1, slots),
            ),
            _ => ForEachAs(
                ESlot(rng.gen::<u8>() % (slots.ent + 1)),
                entity_set(rng, counter,  depth_left-1, slots),
                vec_instruction(rng, counter,  depth_left-1, slots),
            ),            
        }
    }
}
