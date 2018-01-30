use ::std::collections::{HashMap,HashSet};
use magic::*;
use buffs::*;
use std::rc::Rc;
use rand::{Rng, SeedableRng, Isaac64Rng};
use event_context::EventContext;

pub struct Player {
    health: u32,
    health_max: u32,
    mana: u32,
    mana_max: u32,
    buffs: HashMap<Buff, (u8, u32)>,
}

pub struct Projectile {
    bp: Rc<ProjectileBlueprint>,
    pos: Point2D, 
    dir: f32,
    spe: f32,
    sec_left: f32,
}

pub struct Space {
    players: HashMap<Token, (Point2D, Player)>,
    projectiles: HashMap<Token, Projectile>,
    rng: Isaac64Rng,
}


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Token(usize);

impl Space {
    pub fn new() -> Space {
        Space {
            players: HashMap::new(),
            projectiles: HashMap::new(),
            rng: Isaac64Rng::new_unseeded(),
        }
    }

    fn free_token(&mut self) -> Token {
        let mut tok = Token(self.rng.gen());
        loop {
            if !self.players.contains_key(&tok)
            && !self.projectiles.contains_key(&tok) {
                return tok;
            }
            tok.0 += 1;
        }
    }

    pub fn player_enter(&mut self, pt: Point2D, player: Player) -> Token {
        let tok = self.free_token();
        self.players.insert(tok, (pt, player));
        tok
    }

    pub fn player_move(&mut self, token: Token, pt: Point2D) -> bool {
        if let Some(&mut (old_pt, player)) = self.players.get_mut(&token) {
            old_pt = pt;
            true
        } else {
            false
        }
    }   

    pub fn player_leave(&mut self, token: Token) -> Option<(Point2D, Player)> {
        self.players.remove(&token)
    }

    pub fn is_player(&self, token: Token) -> bool {
        self.players.contains_key(&token)
    }

    pub fn is_projectile(&self, token: Token) -> bool {
        self.projectiles.contains_key(&token)
    }

    pub fn eval_discrete(ctx: EventContext,  x: &Discrete) -> f32 {

    }
}


pub struct ConcreteEntitySet {
    players: HashSet<Token>,
    projectiles: HashSet<Token>,
}



impl Player {
    pub fn new(health_max: u32, mana_max: u32) -> Player {
        Player {
            health_max: health_max,
            health: health_max,
            mana_max: mana_max,
            mana: mana_max,
            buffs: HashMap::new(),
        }
    }

    pub fn apply_stacks(&mut self, buff: Buff, stacks: u8, duration: u32) {
        assert!(stacks > 0);
        assert!(duration > 0);
        if let Some(&mut (ref mut old_stacks, ref mut old_duration)) = self.buffs.get_mut(&buff) {
            use buffs::StackingBehaviour::*;
            match stacking_method(buff) {
                Min => {
                    *old_stacks += stacks;
                    if duration < *old_duration {
                        *old_duration = duration;
                    }
                },
                Max => {
                    *old_stacks += stacks;
                    if duration > *old_duration {
                        *old_duration = duration;
                    }
                },
                Replace => {
                    *old_stacks += stacks;
                    *old_duration = duration;
                },
                IfMax => {
                    if duration >= *old_duration {
                        *old_stacks += stacks;
                    }
                },
            }
        }
    }

    pub fn decrement_times(&mut self) {
        for buff in self.buffs.keys().map(|x| *x).collect::<Vec<_>>() {
            let mut remove = false;
            if let Some(&mut (_, ref mut old_duration)) = self.buffs.get_mut(&buff) {
                *old_duration -= 1;
                if *old_duration == 0 {
                    remove = true;
                } 
            }
            if remove {
                self.buffs.remove(&buff);
            }
        }
    }
}


#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Point2D(pub f32, pub f32);

pub fn game_loop() {

}

pub fn initial_Space() -> Space {
    let mut s = Space::new();
    s.put(
        Player::new(PlayerId(0), 100, 100),
        Point2D(0.5,0.5)
    );
    s
}
