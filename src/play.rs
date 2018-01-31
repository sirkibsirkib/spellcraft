use ::std::collections::{HashMap,HashSet};
use magic::*;
use buffs::*;
use std::rc::Rc;
use rand::{Rng, SeedableRng, Isaac64Rng};
use event_context::{EventContext,ContextFor};
use ordermap::OrderSet;

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
    projectiles: HashMap<Token, (Point2D, Projectile)>,
    rng: Isaac64Rng,
}


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Token(usize);
impl Token {
    const NULL: Token = Token(0);

    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

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
            if !tok.is_null()
            && !self.players.contains_key(&tok)
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

    fn point_of(&self, tok: Token) -> Option<Point2D> {
        if let Some(&(pt,_)) = self.players.get(&tok) {
            Some(pt)
        } else if let Some(&(pt,_)) = self.projectiles.get(&tok) {
            Some(pt)
        } else {
            None
        }
    }

    fn eval_discrete(&mut self, ctx: &EventContext,  discrete: &Discrete) -> i32 {
        use magic::Discrete::*;
        match discrete {
            &Const(x) => x,
            &Range(x, y) => (self.rng.gen::<i32>().abs() % (y-x)) + x,
            &WithinPercent(x, y) => ((self.rng.gen::<f32>() * y.0) * (x as f32)) as i32,
            &Div(ref x, ref y) => self.eval_discrete(ctx, x) / self.eval_discrete(ctx, y),
            &Sum(ref x) => x.iter().map(|q| self.eval_discrete(ctx, q)).sum(),
            &Neg(ref x) => -self.eval_discrete(ctx, &x),
            &Mult(ref x) => x.iter().fold(1, |a,b| a * self.eval_discrete(ctx, b)),
            &Max(ref x) => x.iter().fold(1, |a, b| {
                let b = self.eval_discrete(ctx, b);
                if a > b {a} else {b}
            }),
            &Min(ref x) => x.iter().fold(1, |a, b| {
                let b = self.eval_discrete(ctx, b);
                if a < b {a} else {b}
            }),
            &CountStacks(buff, ref ent) => {
                let tok = self.eval_entity(ctx, ent);
                if let Some(&(pt, player)) = self.players.get(&tok) {
                    if let Some(&(stacks, _)) = player.buffs.get(&buff) {
                        stacks as i32
                    } else {
                        0 //no stacks
                    }
                } else if let Some(projectile) = self.projectiles.get(&tok) {
                    // !!!!!!!!!!!!!! TODO
                    0
                } else {
                    0 //no player
                }
            },
            &CountDur(buff, ref ent) => {
                let tok = self.eval_entity(ctx, ent);
                if let Some(&(pt, player)) = self.players.get(&tok) {
                    if let Some(&(_, dur)) = player.buffs.get(&buff) {
                        dur as i32
                    } else {
                        0 //no stacks
                    }
                } else if let Some(projectile) = self.projectiles.get(&tok) {
                    // !!!!!!!!!!!!!! TODO
                    0
                } else {
                    0 //no player
                }
            },
            &Choose(ref x) => {
                if let Some(x) = self.rng.choose(x) {
                    self.eval_discrete(ctx, x)
                } else { 0 }
            },
            &Cardinality(ref eset) => self.eval_entity_set(ctx, eset).cardinality() as i32,
            &LoadFrom(dslot) => *ctx.load(&dslot).unwrap_or(&0),
        }
    }

    fn eval_entity(&mut self, ctx: &EventContext, entity: &Entity) -> Token {
        use magic::Entity::*;
        match entity {
            &LoadEntity(eslot) => *ctx.load(&eslot).unwrap_or(&Token::NULL),
            &FirstOf(ref eset) => self.eval_entity_set(ctx, eset).first(),
            &Choose(ref eset) => self.eval_entity_set(ctx, eset).choose(&mut self.rng),
            &ClosestFrom(ref eset, ref loc) => {
                let ref_pt = self.eval_location(ctx, loc);
                let (mut closest, mut smallest_dist) = (Token::NULL, ::std::f32::MAX);
                for ent_tok in self.eval_entity_set(ctx, eset).0 {
                    if let Some(pt) = self.point_of(ent_tok) {
                        let dist = pt.dist_to(&ref_pt);
                        if dist < smallest_dist {
                            smallest_dist = dist;
                            closest = ent_tok;
                        }
                    }
                }
                closest
            },
        }
    }



    fn eval_entity_set(&mut self, ctx: &EventContext, entity_set: &EntitySet) -> TokenSet {
        use magic::EntitySet::*;
        match entity_set {
            &Nand(ref sets) => ,
            &And(ref sets) => {
                let sets = sets.map(|s| self.eval_entity_set(ctx, s));
                let union = TokenSet::new();
                for s in sets {
                    union.players.union(s.players);
                    union.projectiles.union(s.players);
                }
            },
            &Or(ref sets),
            &Only(ref ent),
            &IsInSlot(eset_slot),
            &WithinRangeOf(ref ent, ref disc),
            &HasMinResource(ref ent, ref res),
            &EnemiesOf(ref ent),
            &AllBut(ref ent),
            &IsHuman,
            &IsProjectile,
            &Empty,
            &Universe,
        }
    }

    fn eval_location(&mut self, ctx: &EventContext, location: &Location) -> Point2D {

    }
}


pub struct TokenSet(Vec<Token>);
impl TokenSet {
    pub fn new() -> Self {
         TokenSet(vec![])
    }
    #[inline]
    pub fn cardinality(&self) -> usize {
        self.0.len()
    }
    pub fn first(&self) -> Token {
        *self.0.iter().nth(0)
        .unwrap_or(&Token::NULL)
    }
    pub fn contains(&self, tok: Token) -> bool {
        self.0.binary_search(&tok).is_ok()
    }
    pub fn remove(&self, tok: Token) -> Option<Token> {
        if let Ok(index) = self.0.binary_search(&tok) {
            Some(self.0.remove(index))
        } else {
            None
        }
    }
    pub fn insert(&mut self, tok: Token) -> bool {
        if let Err(index) = self.0.binary_search(&tok) {
            self.0.insert(index, tok);
            true
        } else {
            false
        }
    }
    pub fn choose<R: Rng>(&self, rng: &mut R) -> Token {
        if let Some(z) = rng.choose(&self.0) {
            *z
        } else {
            Token::NULL
        }
    }
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

macro_rules! sqr {
    ($x:expr) => {{$x*$x}}
}


#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Point2D(pub f32, pub f32);
impl Point2D {
    pub fn dist_to(&self, other: &Self) -> f32 {
        (
            sqr![((self.0 + other.0) as f32)]
            + sqr![((self.1 + other.1) as f32)]
        ).sqrt()
    }
}

pub fn game_loop() {
    let mut space = Space::new();
    let token = space.player_enter(Point2D(0.5, 0.5), Player::new(100, 100));

}
