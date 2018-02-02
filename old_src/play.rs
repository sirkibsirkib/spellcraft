use code::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::collections::HashSet;

use rand::{Rng, Isaac64Rng};

struct BuffStack(u32, u32); // (stacks, sec_remaining)


struct Player {
    coord: Coord,
    spells: Vec<Spell>,
    buffs: HashMap<Buff, Vec<BuffStack>>,
}

impl Player {
    pub fn new(coord: Coord) -> Self {
        Player {
            coord: coord,
            spells: vec![],
            buffs: HashMap::new(),
        }
    }

    pub fn try_cast(&mut self, index: usize) -> bool {
        if let Some(ref mut spell) = self.spells.get_mut(index) {
            true
            //TODO
        } else {
            false
        }
    }
}

struct World {
    players: Vec<Player>,
}

impl World {
    pub fn match_entities(&self, predicate: &Entity) -> HashSet<MatchedEntity> {
        let mut set = HashSet::new();
        set
    }

    pub fn apply_1target_effect(&mut self, rng: &mut Isaac64Rng, arg0: &Entity, effect: &T1Effect) {
        use code::T1Effect::*;
        match effect {
            &All(ref vec_t1) => for x in vec_t1 { self.apply_1target_effect(rng, arg0, x); },
            &Any(ref vec_t1) => { 
                if let Some(choice) = rng.choose(vec_t1) {
                    self.apply_1target_effect(rng, arg0, choice);
                }
            },
            &ITE(ref ent, ref then_vec_t2, ref else_vec_t2) => (), //checks arg0. recurses with arg0
            &EmitProjectile(ref box_bp) => (),
            &AddResource(ref res) => (),
            &MoveTo(ref place) => (),
            &AddFirstArg(ref box_t2, ref ent) => (),
            &AddSecondArg(ref box_t2, ref ent) => (),
            &SwapArgs(ref box_t1) => (),
            &AddHealth(ref discrete) => (),
            &DupArg(ref t2_eff) => (),
            &AddBuffStacks(ref buff, ref disc_stacks, ref disc_dur) => (), //buff, stacks, duration 
            &ReplaceArg(ref ent, ref box_t1) => (),
            &Destroy => (),
            &Nothing => (),
        }
    }

    pub fn apply_2target_effect(&mut self, arg0: &Entity, arg1: &Entity, effect: &T2Effect) {
        
    }
}

struct EntitySet {
    HashSet<MatchedEntity
}

struct Projectile {
    bp: Arc<ProjectileBlueprint>,
    coord: Coord,
}

pub struct Coord(f32, f32);

enum MatchedEntity<'a> {
    Player(&'a Player),
    Projectile(&'a Projectile),
}

impl<'a> PartialEq for MatchedEntity<'a> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            &MatchedEntity::Player(x) => {
                if let &MatchedEntity::Player(y) = other {
                    x as * const Player == (y as *const Player)
                } else { false }               
            },
            &MatchedEntity::Projectile(x) => {
                if let &MatchedEntity::Projectile(y) = other {
                    x as * const Projectile == (y as *const Projectile)
                } else { false } 
            }
        }
    }
}

impl<'a> ::std::hash::Hash for MatchedEntity<'a> {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        match self {
            &MatchedEntity::Player(x) => {
                (x as *const Player).hash(state);
            },
            &MatchedEntity::Projectile(x) => {
                (x as *const Projectile).hash(state);
            },
        }
        
    }
}

impl<'a> Eq for MatchedEntity<'a> {}

// impl EntityEvals for Player {
//     fn evaluate_self(&self, predicate: &Entity) -> bool {
//         use ::code::Entity::*;
//         match predicate {
//             &And(ref ent_vec) => {
//                 for e in ent_vec.iter() {
//                     if !self.evaluate_self(e) { return false }
//                 }
//                 true
//             },
//             &Or(ref ent_vec) => {
//                 for e in ent_vec.iter() {
//                     if self.evaluate_self(e) { return true }
//                 }
//                 false
//             },
//             &Nand(ref ent_vec) => {
//                 for e in ent_vec.iter() {
//                     if self.evaluate_self(e) { return false }
//                 }
//                 true
//             },

//             &Argument => ,
//             &IsTheCaster => ,
//             &IsAPlayer => ,
//             &IsProjectile => ,
//             &HasResourceMin(ref resource) => ,
//             &HasResourceMax(ref resource) => ,
//             &HasResourceExact(ref resource) => ,
//             &HasWithinRange(ref discrete, ref entbox) => ,
//         }
//     }
// }

// trait EntityEvals {
//     fn evaluate_self(&self, predicate: &Entity) -> bool;
// }