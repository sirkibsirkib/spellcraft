use ::std::collections::{HashMap,HashSet};
use magic::*;
use buffs::*;
use std::rc::Rc;
use rand::{Rng,Isaac64Rng,SeedableRng};
use event_context::{EventContext,ContextFor};
use movement_2d::*;
use piston_window::*;
use super::piston_window::{G2dTexture,Texture,TextureSettings,Flip};
use wasd_set::{WasdSet,WasdDirection};
use find_folder;
use generate;

const UPDATES_PER_SEC: u64 = 30;
const RENDERS_PER_SEC: u64 = 30;


pub struct Projectile {
    bp: Rc<ProjectileBlueprint>,
    caster: Token,
    pos: Point, 
    sec_left: f32,
    velocity: Vector,
}

#[allow(dead_code)]
pub struct Space {
    players: HashMap<Token, (Point, Player)>,
    projectiles: HashMap<Token, (Point, Projectile)>,
    rng: Isaac64Rng,
    token_players: TokenSet,
    token_projectiles: TokenSet,
    token_universe: TokenSet,
}


#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Token(usize);
impl Token {
    const NULL: Token = Token(0);

    #[inline]
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

type IRng = Isaac64Rng;

impl Space {
    const TICK_PERIOD: f32 = 1.0 / UPDATES_PER_SEC as f32;

    pub fn new() -> Space {
        Space {
            players: HashMap::new(),
            projectiles: HashMap::new(),
            rng: Isaac64Rng::new_unseeded(),

            //optimization
            token_universe: TokenSet::new(),
            token_players: TokenSet::new(),
            token_projectiles: TokenSet::new(),
        }
    }

    fn tick(&mut self) {
        let mut rm_tokens: Vec<Token> = vec![];

        //PLAYERS
        let mut rm_buff = vec![];
        for (&tok, &mut (ref mut pt, ref mut player)) in self.players.iter_mut() {
            for (&buff, &mut (ref mut stacks, ref mut left)) in player.buffs.iter_mut() {
                *left -= Self::TICK_PERIOD;
                if *left <= 0. {
                    rm_buff.push(buff); // buff dur falloff
                }
            }
            for buff in rm_buff.drain(..) {
                player.buffs.remove(&buff); // buff complete falloff
            }
            //TODO move all players 
            pt.apply_vector(&player.velocity);

            //Decelerate all players
            player.velocity *= 0.8;
            player.velocity.slow_by(1.0);
        }
        for token in rm_tokens.drain(..) {
            self.players.remove(&token);
        }

        // PROJECTILES
        for (&tok, &mut (ref mut pt, ref mut proj)) in self.projectiles.iter_mut() {
            // tick down
            proj.sec_left -= Space::TICK_PERIOD;
            if proj.sec_left <= 0.0 {
                rm_tokens.push(tok);
            }

            // move
            pt.apply_vector(&proj.velocity);

            // collisions
        }
        for token in rm_tokens.drain(..) {
            self.projectiles.remove(&token);
        }
    }

    pub fn add_velocity_to(&mut self, token: Token, velocity: Vector) -> bool {
        self.add_velocity_to_player(token, velocity)
        || self.add_velocity_to_projectile(token, velocity)
    }

    pub fn add_velocity_to_player(&mut self, token: Token, velocity: Vector) -> bool {
        if let Some(&mut (_, ref mut player)) = self.players.get_mut(&token) {
            player.velocity += velocity;
            true
        } else { false }
    }

    pub fn add_velocity_to_projectile(&mut self, token: Token, velocity: Vector) -> bool {
        if let Some(&mut (_, ref mut proj)) = self.projectiles.get_mut(&token) {
            proj.velocity += velocity;
            true
        } else { false }
    }

    pub fn pt_of_player(&self, token: Token) -> Option<Point> {
        self.players.get(&token).map(|x| x.0)
    }

    fn spawn_projectile(&mut self, caster: Token, spawn_at: Point, cursor: Point, bp: Rc<ProjectileBlueprint>) {
        let mut ctx = EventContext::new();
        ctx.define(ESlot(0), caster);
        let lifetime = self.eval_discrete(&mut self.rng.clone(), &ctx, &(&bp).lifetime) as f32;
        let projectile = Projectile {
            bp: bp.clone(),
            caster: caster,
            pos: spawn_at, //TODO allow projectiles to spawn elsewhere 
            sec_left: lifetime,
            velocity: Vector::NULL,
        };

        let tok = self.free_token();
        let pt = self.point_of(caster).unwrap();
        self.projectiles.insert(tok, (pt, projectile));
        self.token_universe.insert(tok);
        self.token_projectiles.insert(tok);

        let mut ctx = EventContext::new();
        ctx.define(ESlot(0), caster);
        ctx.define(ESlot(1), tok);
        ctx.define(LSlot(0), cursor);

        let mut rng2 = self.rng.clone();
        for ins in bp.on_create.iter() {
            self.execute_instruction(&mut rng2, &mut ctx, ins);
        }
    }

    fn free_token(&mut self) -> Token {
        let mut tok = Token(self.rng.gen());
        loop {
            if !tok.is_null()
            && !self.token_universe.contains(tok) {
                break;
            }
            tok.0 += 1;
        }
        tok
    }

    pub fn player_enter(&mut self, pt: Point, player: Player) -> Token {
        let tok = self.free_token();
        self.players.insert(tok, (pt, player));
        self.token_universe.insert(tok);
        self.token_players.insert(tok);
        tok
    }

    pub fn spell_of(&self, caster_token: Token, spell_index: usize) -> Option<Rc<Spell>> {
        if let Some(&(ref _pt, ref player)) = self.players.get(&caster_token) {
            if let Some(player_spell) = player.spells.get(spell_index) {
                return Some(player_spell.clone());
            }
        }
        None
    }


    pub fn player_cast(&mut self, caster_token: Token, spell_index: usize, cursor_point: Point) {
        println!("player_cast");
        let mut rng2 = self.rng.clone();
        let spell: Option<Rc<Spell>> = self.spell_of(caster_token, spell_index);
        if spell == None {
            println!("no spell in slot {:?}", spell_index);
            return;
        }
        let spell = spell.unwrap();
        let mut ctx = EventContext::new();
        ctx.e.insert(ESlot(0), caster_token);
        ctx.l.insert(LSlot(0), cursor_point);
        println!("{:#?}", &spell);    
        if self.eval_condition(&mut rng2, &ctx, &spell.requires) {
            println!("Condition met!");
        } else {
            println!("Condition not met!");
            return;
        }
        let consume = {
            spell.consumes
            .iter()
            .map(|r| self.eval_resource(&mut rng2, &ctx, r))
            .collect::<Vec<_>>()
        };
        println!("consuming... {:#?}", &consume);
        let x = self.players.get_mut(&caster_token).map(
            |&mut (_, ref mut player)|
            player.try_remove_resources(&consume[..])
        );
        if let Some(true) = x {
            println!("consume success!");
            for ins in spell.on_cast.iter() {
                self.execute_instruction(&mut rng2, &mut ctx, ins);
            }
        } else {
            println!("consume failure!");
        }
    }

    pub fn move_to(&mut self, token: Token, pt: Point) -> bool {
        if let Some(&mut (ref mut old_pt, _)) = self.players.get_mut(&token) {
            *old_pt = pt;
            true
        } else {
            false
        }
    }   

    pub fn player_leave(&mut self, token: Token) -> Option<(Point, Player)> {
        self.token_universe.remove(token);
        self.token_players.remove(token);
        self.players.remove(&token)
    }

    pub fn is_player(&self, token: Token) -> bool {
        self.players.contains_key(&token)
    }

    pub fn is_projectile(&self, token: Token) -> bool {
        self.projectiles.contains_key(&token)
    }

    fn point_of(&self, tok: Token) -> Option<Point> {
        if let Some(&(pt,_)) = self.players.get(&tok) {
            Some(pt)
        } else if let Some(&(pt,_)) = self.projectiles.get(&tok) {
            Some(pt)
        } else {
            None
        }
    }

    fn execute_instruction(&mut self, rng: &mut IRng, ctx: &mut EventContext, ins: &Instruction) {
        use magic::Instruction::*;
        println!("Executing ... {:?}", ins);
        match ins {
            &Define(ref def) => self.execute_defintion(rng, ctx, def),
            &ITE(ref cond, ref then, ref els) => {
                if self.eval_condition(rng, ctx, cond) {
                    for i in then {
                        self.execute_instruction(rng, ctx, i);
                    }
                } else {
                    for i in els {
                        self.execute_instruction(rng, ctx, i);
                    }
                }
            },
            &CallWith(ref def, ref ins) => {
                let ctx_was = ctx.clone();
                self.execute_defintion(rng, ctx, def);
                for i in ins {
                    self.execute_instruction(rng, ctx, i);
                }
                *ctx = ctx_was;
            },
            &ForEachAs(slot, ref set, ref ins) => {
                let set = self.eval_entity_set(rng, ctx, set);
                for &tok in set.0.iter() {
                    ctx.define(slot, tok);
                    for i in ins {
                        self.execute_instruction(rng, ctx, i);
                    }
                }
            },
            &DestroyWithoutEvent(ref ent) => {
                let tok = self.eval_entity(rng, ctx, ent);
                self.destroy(tok, false);
            },
            &Destroy(ref ent) => {
                let tok = self.eval_entity(rng, ctx, ent);
                self.destroy(tok, true);
            },
            &MoveEntity(ref ent, ref loc) => {
                let pt = self.eval_location(rng, ctx, loc);
                let token = self.eval_entity(rng, ctx, ent);
                self.move_to(token, pt);
            },
            &AddResource(ref ent, ref rsrc) => {
                let token = self.eval_entity(rng, ctx, ent);

            },
            &AddVelocity(ref ent, ref dir, ref disc) => { // last arg is "speed"
                let tok = self.eval_entity(rng, ctx, ent);
                let f = self.eval_direction(rng, ctx, dir);
                let d = self.eval_discrete(rng, ctx, disc);
                let vel = Vector::new_from_directional(f, d as f32);
                self.add_velocity_to(tok, vel);
            },
            &SpawnProjectileAt(ref rc_proj, ref loc) => {
                let spawn_loc = self.eval_location(rng, ctx, loc);
                if let (Some(&token), Some(&cursor_loc)) = (ctx.load(&ESlot(0)), ctx.load(&LSlot(0))) {
                    self.spawn_projectile(token, spawn_loc, cursor_loc, rc_proj.clone()); 
                }
            },
            &Nothing => (),
        }
    }

    fn destroy(&mut self, token: Token, trigger_event: bool) -> bool {
        //TODO trigger destroy events
        self.players.remove(&token).is_some()
        || self.projectiles.remove(&token).is_some()
    }

    fn execute_defintion(&mut self, rng: &mut IRng, ctx: &mut EventContext, def: &Definition) {
        use magic::Definition::*;
        match def {
            &ESet(s, ref eset) => {
                let x = self.eval_entity_set(rng, ctx, eset);
                ctx.define(s, x)
            },
            &E(s, ref e) => {
                let x = self.eval_entity(rng, ctx, e);
                ctx.define(s, x)
            },
            &D(s, ref d) => {
                let x = self.eval_discrete(rng, ctx, d);
                ctx.define(s, x)
            },
            &L(s, ref l) => {
                let x = self.eval_location(rng, ctx, l);
                ctx.define(s, x)
            },
        }
    }



    fn eval_resource(&self, rng: &mut IRng, ctx: &EventContext, resource: &Resource) -> ConcreteResource {
        use magic::Resource::*;
        match resource {
            &Mana(ref x) => ConcreteResource::Mana(
                self.eval_discrete(rng, &ctx, x)
            ),
            &Health(ref x) => ConcreteResource::Health(
                self.eval_discrete(rng, &ctx, x)
            ),
            &BuffStacks(b, ref x) => ConcreteResource::BuffStacks(
                b,
                self.eval_discrete(rng, &ctx, x) as i8,
            ),
        }
    }

    fn eval_direction(&self, rng: &mut IRng, ctx: &EventContext, direction: &Direction) -> f32 {
        use magic::Direction::*;
        match direction {
            &TowardLocation(ref from, ref to) => {
                let from = self.eval_location(rng, ctx, from);
                let to = self.eval_location(rng, ctx, to);
                if from != Point::NULL && to != Point::NULL {
                    from.direction_to(&to)
                } else { 0.0 }
            },
            &ConstRad(new_f32) => new_f32.0,
            &BetweenRad(a, b) => a.0 + (rng.gen::<f32>() * (b.0 - a.0)),
            &Choose(ref dirs) => {
                rng.choose(dirs)
                .map(|d| self.eval_direction(rng, ctx, d))
                .unwrap_or(0.0)
            },
            &ChooseWithinRadOf(ref dir, ref new_f32) => {
                let mut val = rng.gen::<f32>() * new_f32.0;
                if rng.gen() {val *= -1.0}
                val + self.eval_direction(rng, ctx, dir)
            },
        }
    }

    fn eval_discrete(&self, rng: &mut IRng, ctx: &EventContext, discrete: &Discrete) -> i32 {
        use magic::Discrete::*;
        match discrete {
            &Const(x) => x,
            &Range(x, y) => (rng.gen::<i32>().abs() % (y-x)) + x,
            &WithinPercent(ref x, ref y) => ((rng.gen::<f32>() * (*y).0) * (*x as f32)) as i32,
            &Div(ref x, ref y) => self.eval_discrete(rng, ctx, x) / self.eval_discrete(rng, ctx, y),
            &Sum(ref x) => x.iter().map(|q| self.eval_discrete(rng, ctx, q)).sum(),
            &Neg(ref x) => -self.eval_discrete(rng, ctx, &x),
            &Mult(ref x) => x.iter().fold(1, |a,b| a * self.eval_discrete(rng, ctx, b)),
            &Max(ref x) => x.iter().fold(1, |a, b| {
                let b = self.eval_discrete(rng, ctx, b);
                if a > b {a} else {b}
            }),
            &Min(ref x) => x.iter().fold(1, |a, b| {
                let b = self.eval_discrete(rng, ctx, b);
                if a < b {a} else {b}
            }),
            &CountStacks(buff, ref ent) => {
                let tok = self.eval_entity(rng, ctx, ent);
                if let Some(&(_, ref player)) = self.players.get(&tok) {
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
                let tok = self.eval_entity(rng, ctx, ent);
                if let Some(&(_, ref player)) = self.players.get(&tok) {
                    if let Some(&(_, dur)) = player.buffs.get(&buff) {
                        dur as i32
                    } else {
                        0 //no stacks
                    }
                } else if let Some(_projectile) = self.projectiles.get(&tok) {
                    // !!!!!!!!!!!!!! TODO
                    0
                } else {
                    0 //no player
                }
            },
            &Choose(ref x) => {
                if let Some(x) = rng.choose(x) {
                    self.eval_discrete(rng, ctx, x)
                } else { 0 }
            },
            &Cardinality(ref eset) => self.eval_entity_set(rng, ctx, eset).cardinality() as i32,
            &LoadFrom(dslot) => *ctx.load(&dslot).unwrap_or(&0),
        }
    }

    fn eval_entity(&self, rng: &mut IRng, ctx: &EventContext, entity: &Entity) -> Token {
        use magic::Entity::*;
        match entity {
            &LoadEntity(eslot) => *ctx.load(&eslot).unwrap_or(&Token::NULL),
            &FirstOf(ref eset) => self.eval_entity_set(rng, ctx, eset).first(),
            &Choose(ref eset) => self.eval_entity_set(rng, ctx, eset).choose(rng),
            &ClosestFrom(ref eset, ref loc) => {
                let ref_pt = self.eval_location(rng, ctx, loc);
                let (mut closest, mut smallest_dist) = (Token::NULL, ::std::f32::MAX);
                for ent_tok in self.eval_entity_set(rng, ctx, eset).0 {
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

    fn eval_entity_set(&self, rng: &mut IRng, ctx: &EventContext, entity_set: &EntitySet) -> TokenSet {
        use magic::EntitySet::*;
        match entity_set {
            &None(ref sets) => {
                let tokens = self.token_universe.0.iter().map(|x| *x).collect::<Vec<_>>();
                let sets = sets.iter().map(|s| self.eval_entity_set(rng, ctx, s)).collect::<Vec<_>>();
                let mut ret = TokenSet::new();
                for tok in tokens {
                    let mut found = false;
                    for s in sets.iter() {
                        if s.contains(tok) {
                            found = true;
                            break;
                        }
                    }
                    if ! found {
                        ret.insert(tok);
                    }
                }
                ret
            },
            &And(ref sets) => {
                let sets = sets.iter().map(|s| self.eval_entity_set(rng, ctx, s)).collect::<Vec<_>>();
                let mut ret = self.token_universe.clone();
                for &tok in self.token_universe.0.iter() {
                    let mut found = false;
                    for s in sets.iter() {
                        if s.contains(tok) {
                            found = true;
                            break;
                        }
                    }
                    if ! found {
                        ret.insert(tok);
                    }
                }
                ret
            },
            &Or(ref sets) => {
                let mut ret = self.token_universe.clone();
                let uni_clone = self.token_universe.clone();
                for tok in uni_clone.0 {
                    for s in sets.iter().map(|s| self.eval_entity_set(rng, ctx, s)) {
                        if s.contains(tok) {
                            ret.insert(tok);
                            break;
                        }
                    }
                }
                ret
            },
            &Only(ref ent) => {
                let mut s = TokenSet::new();
                s.insert(self.eval_entity(rng, ctx, ent));
                s   
            },
            &IsInSlot(ref eset_slot) => ctx.load(eset_slot).expect("NOTHING IN SLOT").clone(), //TODO crash on bad load everywhere else too
            &WithinRangeOf(ref ent, ref disc) => {
                let e = self.eval_entity(rng, ctx, ent);
                let ref_loc = self.point_of(e).expect("SHET");
                let thresh = self.eval_discrete(rng, ctx, disc) as f32;
                let mut s = TokenSet::new();
                for &tok in self.token_universe.0.iter() {
                    if ref_loc.dist_to(& self.point_of(tok).expect("bugger")) < thresh {
                        s.insert(tok);
                    }
                }
                s
            },
            &HasMinResource(ref res) => {
                //TODO handle projectiles
                let mut ret = TokenSet::new();
                for tok in self.token_players.0.iter().map(|x| *x).collect::<Vec<_>>() {
                    use magic::Resource::*; //TODO make resource more powerful
                    match res {
                        &Mana(ref x) => {
                            let need = self.eval_discrete(rng, ctx, x);
                            if let Some(&(_, ref player)) = self.players.get(&tok) {
                                if player.mana as i32 >= need {
                                    ret.insert(tok);
                                }
                            }
                        },
                        &Health(ref x) => {
                            let need = self.eval_discrete(rng, ctx, x);
                            if let Some(&(_, ref player)) = self.players.get(&tok) {
                                if player.health as i32 >= need {
                                    ret.insert(tok);
                                }
                            }
                        },
                        &BuffStacks(buff, ref disc) => {
                            let need = self.eval_discrete(rng, ctx, disc);
                            if let Some(&(_, ref player)) = self.players.get(&tok) {
                                let cond = if let Some(&(stacks, _)) = player.buffs.get(&buff) {
                                    stacks as i32 >= need
                                } else { 0 >= need };
                                if cond {
                                    ret.insert(tok);
                                } 
                            }
                        },
                    }
                }
                ret
            },
            &EnemiesOf(ref ent) => {
                let mut s = self.token_players.clone();
                s.remove(self.eval_entity(rng, ctx, ent));
                s
            },
            &AllBut(ref ent) => {
                let mut s = self.token_universe.clone();
                s.remove(self.eval_entity(rng, ctx, ent));
                s
            },
            &IsHuman => self.token_players.clone(),
            &IsProjectile => self.token_projectiles.clone(),
            &Empty => TokenSet::new(),
            &Universe => self.token_universe.clone(),
        }
    }

    fn eval_condition(&self, rng: &mut IRng, ctx: &EventContext, condition: &Condition) -> bool {
        use magic::Condition::*;
        match condition {
            &Nand(ref conds) => !conds.iter().map(|x| self.eval_condition(rng, ctx, x)).fold(true, |a,b| a&&b),
            &And(ref conds) => conds.iter().map(|x| self.eval_condition(rng, ctx, x)).fold(true, |a,b| a&&b),
            &Or(ref conds) => conds.iter().map(|x| self.eval_condition(rng, ctx, x)).fold(true, |a,b| a||b),
            &Top => true,
            &Bottom => false,
            &Equals(ref disc_a, ref disc_b) => self.eval_discrete(rng, ctx, disc_a) == self.eval_discrete(rng, ctx, disc_b),
            &LessThan(ref disc_a, ref disc_b) => self.eval_discrete(rng, ctx, disc_a) < self.eval_discrete(rng, ctx, disc_b),
            &MoreThan(ref disc_a, ref disc_b) => self.eval_discrete(rng, ctx, disc_a) > self.eval_discrete(rng, ctx, disc_b),
            &EntitySetCmp(ref esetcmp) => self.eval_entity_set_cmp(rng, ctx, esetcmp),
        }
    }

    fn eval_entity_set_cmp(&self, rng: &mut IRng, ctx: &EventContext, ent_set_cmp: &EntitySetCmp) -> bool {
        use magic::EntitySetCmp::*;
        match ent_set_cmp {
            &Nand(ref v) => !v.iter().map(|x| self.eval_entity_set_cmp(rng, ctx, x)).fold(true, |a,b| a&&b),
            &And(ref v) => v.iter().map(|x| self.eval_entity_set_cmp(rng, ctx, x)).fold(true, |a,b| a&&b),
            &Or(ref v) => v.iter().map(|x| self.eval_entity_set_cmp(rng, ctx, x)).fold(true, |a,b| a||b),
            &Subset(ref a, ref b) => {
                let a = self.eval_entity_set(rng, ctx, a);
                let b = self.eval_entity_set(rng, ctx, b);
                let mut violated = false;
                for e in a.0.iter() {
                    if !b.0.contains(e) {
                        violated = true;
                        break;
                    }
                }
                !violated
            },
            &Superset(ref a, ref b) => {
                let a = self.eval_entity_set(rng, ctx, a);
                let b = self.eval_entity_set(rng, ctx, b);
                let mut violated = false;
                for e in b.0.iter() {
                    if !a.0.contains(e) {
                        violated = true;
                        break;
                    }
                }
                !violated
            },
            &Equal(ref a, ref b) => {
                self.eval_entity_set(rng, ctx, a) == self.eval_entity_set(rng, ctx, b)
            },
            &Contains(ref eset, ref e) => {
                let set = self.eval_entity_set(rng, ctx, eset);
                let ent = self.eval_entity(rng, ctx, e);
                ent != Token::NULL && set.contains(ent)
            },
        }
    }

    fn eval_location(&self, rng: &mut IRng, ctx: &EventContext, location: &Location) -> Point {
        use magic::Location::*;
        match location {
            &AtEntity(ref ent) => {
                let e = self.eval_entity(rng, ctx, ent);
                self.point_of(e).expect("UH OH")
            },
            &Midpoint(ref locs) => {
                Point::midpoint(
                    & (
                        locs.iter()
                        .map(|x| self.eval_location(rng, ctx, x))
                        .collect::<Vec<_>>()
                    )
                ).unwrap_or(Point::NULL)
            },
            &Choose(ref locs) => {
                if let Some(x) = rng.choose(locs) {
                    self.eval_location(rng, ctx, x)
                } else {
                    Point::NULL
                }
            },
            &LoadLocation(lslot) => {
                *ctx.load(&lslot).expect("lordy")   
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
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
    pub fn remove(&mut self, tok: Token) -> Option<Token> {
        if let Ok(index) = self.0.binary_search(&tok) {
            Some(self.0.remove(index))
        } else {
            None
        }
    }
    pub fn insert(&mut self, tok: Token) -> bool {
        if tok == Token::NULL { return false }
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


#[derive(Debug)]
pub struct Player {
    health: i32,
    health_max: u32,
    mana: i32,
    mana_max: u32,
    buffs: HashMap<Buff, (u8, f32)>,
    velocity: Vector,
    spells: Vec<Rc<Spell>>,
}

impl Player {
    pub fn new(health_max: u32, mana_max: u32) -> Player {
        Player {
            health_max: health_max,
            health: health_max as i32,
            mana_max: mana_max,
            mana: mana_max as i32,
            buffs: HashMap::new(),
            velocity: Vector::NULL,
            spells: Vec::new(),
        }
    }

    pub fn add_spell(&mut self, spell: Spell) {
        self.spells.push(Rc::new(spell));
    }

    // returns true iff successful. only removes any resources if true.
    pub fn try_remove_resources(&mut self, r_left: &[ConcreteResource]) -> bool {
        let mut total_mana = 0;
        let mut total_health = 0;
        let mut total_buffs = HashMap::new();
        //aggregate needed resources
        for r in r_left {
            use self::ConcreteResource::*;
            match r {
                &Mana(x) => total_mana += x,
                &Health(x) => total_health += x,
                &BuffStacks(buff, x) => {
                    if !total_buffs.contains_key(&buff) {
                        total_buffs.insert(buff, x);
                    } else {
                        let val = total_buffs.get_mut(&buff).unwrap();
                        *val += x; 
                    };
                },
            };
        };
        //if player has aggregated resources
        if self.mana >= total_mana
        && self.health >= total_health
        && total_buffs.iter()
                .filter(|&(_, v)| *v >= 0)
                .fold(true, |a, (k, v)| a && self.has_min_stacks(*k, *v as u8)) {

            self.mana -= total_mana;
            self.health -= total_health;
            for (k, v) in total_buffs {
                if v < 0 {
                    //TODO adding a buff
                } else {
                    self.forcibly_remove_buff(k, v as u8);
                }
            };
            true
        } else { false }
    }

    pub fn forcibly_remove_buff(&mut self, buff: Buff, stacks: u8) -> bool {
        if stacks == 0 { return self.buffs.contains_key(&buff) }
        let mut removed_all = false;
        if let Some(&mut (ref mut s, _)) = self.buffs.get_mut(&buff) {
            if *s < stacks {
                removed_all = true;
            } else { *s -= stacks }
        } else { return false }
        if removed_all {
            self.buffs.remove(&buff);
            //TODO removal effect
            true
        } else { false }
    }

    pub fn has_min_stacks(&self, buff: Buff, stacks: u8) -> bool {
        if stacks == 0 { return true }
        if let Some(&(ref s, ref _dur)) = self.buffs.get(&buff) {
            *s >= stacks
        } else {
            false
        }
    }

    pub fn apply_stacks(&mut self, buff: Buff, stacks: u8, duration: f32) {
        assert!(stacks > 0);
        assert!(duration > 0.);
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
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ConcreteResource {
    Mana(i32),
    Health(i32),
    BuffStacks(Buff, i8),
}


pub fn game_loop() {
    let mut me = Player::new(100, 100);
    let mut rng = Isaac64Rng::new_unseeded();
    let mut spells = 0;
    while spells < 10 {
        let (spell, complexity) = generate::spell(4, &mut rng);
        if 5 <= complexity && complexity <= 35 {
            me.add_spell(spell);
            spells += 1;
        }
    }
    let mut space = Space::new();
    let token = space.player_enter(
        Point(200., 100.),
        me
    );
    let mut window = init_window();

    let mut screen_pt: [f64;2] = [0., 0.];
    let mut space_pt: Point = Point(0., 0.);
    let mut wasd_set = WasdSet::new(false);
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let sprites = Sprites {
        wizard: Sprite {
            texture: Texture::from_path(
                &mut window.factory,
                assets.join("wizard.png"),
                Flip::None,
                &TextureSettings::new()
            ).unwrap(),
            center: (30,30),
        },
        fireball: Sprite {
            texture: Texture::from_path(
                &mut window.factory,
                assets.join("fireball.png"),
                Flip::None,
                &TextureSettings::new()
            ).unwrap(),
            center: (30,30),
        },
    };

    while let Some(e) = window.next() {
        if let Some(_) = e.update_args() {
            use self::WasdDirection::*;
            use ::std::f32::consts::PI;
            let mut flag = false;
            let mut dir = match wasd_set.direction() {
                Nothing => {flag = true; 0.0},
                W => PI*1.5,
                A => PI*1.0,
                S => PI*0.5,
                D => PI*0.0,
                WA => PI*1.25,
                WD => PI*1.75,
                SA => PI*0.75,
                SD => PI*0.25,
            };
            if !flag {
                // if let Some(pt) = space.pt_of_player(token) {
                //     let facing = pt.direction_to(&space_pt);
                //     dir += facing - (PI * 0.5);
                // }
                space.add_velocity_to(token, Vector::new_from_directional(dir, 4.0));
            }
            space.tick();
        }
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, | _ , graphics| clear([0.0; 4], graphics));
            render_space(&e, &mut window, &space, &sprites);
        }
        if let Some(z) = e.mouse_cursor_args() {
            screen_pt = z;
            space_pt = Point(screen_pt[0] as f32, screen_pt[1] as f32);
        }
        if let Some(button) = e.press_args() {
            match button {
                Button::Mouse(MouseButton::Left) => (), //TODO click at <mouse_at>
                Button::Keyboard(Key::W) => wasd_set.press_w(),
                Button::Keyboard(Key::A) => wasd_set.press_a(),
                Button::Keyboard(Key::S) => wasd_set.press_s(),
                Button::Keyboard(Key::D) => wasd_set.press_d(),
                Button::Keyboard(Key::D0) => space.player_cast(token, 0, space_pt),
                Button::Keyboard(Key::D1) => space.player_cast(token, 1, space_pt),
                Button::Keyboard(Key::D2) => space.player_cast(token, 2, space_pt),
                Button::Keyboard(Key::D3) => space.player_cast(token, 3, space_pt),
                Button::Keyboard(Key::D4) => space.player_cast(token, 4, space_pt),
                Button::Keyboard(Key::D5) => space.player_cast(token, 5, space_pt),
                Button::Keyboard(Key::D6) => space.player_cast(token, 6, space_pt),
                Button::Keyboard(Key::D7) => space.player_cast(token, 7, space_pt),
                Button::Keyboard(Key::D8) => space.player_cast(token, 8, space_pt),
                Button::Keyboard(Key::D9) => space.player_cast(token, 9, space_pt),
                x => (),//TODO
            }
        }
        if let Some(button) = e.release_args() {
            match button {
                Button::Mouse(MouseButton::Left) => (), //TODO release at <mouse_at>
                Button::Keyboard(Key::W) => wasd_set.release_w(),
                Button::Keyboard(Key::A) => wasd_set.release_a(),
                Button::Keyboard(Key::S) => wasd_set.release_s(),
                Button::Keyboard(Key::D) => wasd_set.release_d(),
                x => (),//TODO
            }
        }
    }
}


fn init_window() -> PistonWindow {
    let mut window: PistonWindow = WindowSettings::new("Spellcraft", ((600) as u32, (500) as u32))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let event_settings = EventSettings {
        max_fps: RENDERS_PER_SEC,
        ups: UPDATES_PER_SEC,
        ups_reset: 2,
        swap_buffers: true,
        bench_mode: false,
        lazy: false,
    };
    window.set_event_settings(event_settings);
    window
}


struct Sprites {
    wizard: Sprite,
    fireball: Sprite,
}


struct Sprite {
    center: (u32, u32), //TODO
    texture: G2dTexture,
}



fn render_space<E>(
            event : &E,
            window : &mut PistonWindow,
            space: &Space,
            sprites: &Sprites,
) where E : GenericEvent {
    window.draw_2d(event, |c, g| {
        let wiz_sprite = &sprites.wizard;
        let fireball = &sprites.fireball;
        for (&tok, &(ref pt, ref player)) in space.players.iter() {
            image(&wiz_sprite.texture, c.transform
                .trans(
                    pt.0 as f64 - (wiz_sprite.center.0 as f64),
                    pt.1 as f64 - (wiz_sprite.center.1 as f64),
                ).zoom(0.3), g);
        }
        for (&tok, &(ref pt, ref player)) in space.projectiles.iter() {
            image(&fireball.texture, c.transform
                .trans(
                    pt.0 as f64 - (fireball.center.0 as f64),
                    pt.1 as f64 - (fireball.center.1 as f64),
                ).zoom(0.3), g);
        }
    });
}