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

    pub fn add_velocity_to_player(&mut self, token: Token, velocity: Vector) -> bool {
        if let Some(&mut (_, ref mut player)) = self.players.get_mut(&token) {
            player.velocity += velocity;
            true
        } else {
            false
        }
    }

    pub fn pt_of_player(&self, token: Token) -> Option<Point> {
        self.players.get(&token).map(|x| x.0)
    }

    fn spawn_projectile(&mut self, caster: Token, bp: Rc<ProjectileBlueprint>) {
        let mut ctx = EventContext::new();
        ctx.define(ESlot(0), caster);
        let lifetime = self.eval_discrete(&ctx, &(&bp).lifetime) as f32;
        let projectile = Projectile {
            bp: bp.clone(),
            caster: caster,
            pos: self.point_of(caster).unwrap(), //TODO allow projectiles to spawn elsewhere 
            sec_left: lifetime,
            velocity: Vector::NULL,
        };

        let tok = self.free_token();
        let pt = self.point_of(caster).unwrap();
        self.projectiles.insert(tok, (pt, projectile));
        self.token_universe.insert(tok);
        self.token_projectiles.insert(tok);
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

    pub fn player_move(&mut self, token: Token, pt: Point) -> bool {
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

    fn eval_discrete(&mut self, ctx: &EventContext,  discrete: &Discrete) -> i32 {
        use magic::Discrete::*;
        match discrete {
            &Const(x) => x,
            &Range(x, y) => (self.rng.gen::<i32>().abs() % (y-x)) + x,
            &WithinPercent(ref x, ref y) => ((self.rng.gen::<f32>() * (*y).0) * (*x as f32)) as i32,
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
                let tok = self.eval_entity(ctx, ent);
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
            &None(ref sets) => {
                let tokens = self.token_universe.0.iter().map(|x| *x).collect::<Vec<_>>();
                let sets = sets.iter().map(|s| self.eval_entity_set(ctx, s)).collect::<Vec<_>>();
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
                let sets = sets.iter().map(|s| self.eval_entity_set(ctx, s)).collect::<Vec<_>>();
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
                    for s in sets.iter().map(|s| self.eval_entity_set(ctx, s)) {
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
                s.insert(self.eval_entity(ctx, ent));
                s   
            },
            &IsInSlot(ref eset_slot) => ctx.load(eset_slot).expect("NOTHING IN SLOT").clone(), //TODO crash on bad load everywhere else too
            &WithinRangeOf(ref ent, ref disc) => {
                let e = self.eval_entity(ctx, ent);
                let ref_loc = self.point_of(e).expect("SHET");
                let thresh = self.eval_discrete(ctx, disc) as f32;
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
                            let need = self.eval_discrete(ctx, x);
                            if let Some(&(_, ref player)) = self.players.get(&tok) {
                                if player.mana as i32 >= need {
                                    ret.insert(tok);
                                }
                            }
                        },
                        &Health(ref x) => {
                            let need = self.eval_discrete(ctx, x);
                            if let Some(&(_, ref player)) = self.players.get(&tok) {
                                if player.health as i32 >= need {
                                    ret.insert(tok);
                                }
                            }
                        },
                        &BuffStacks(buff, ref disc) => {
                            let need = self.eval_discrete(ctx, disc);
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
                s.remove(self.eval_entity(ctx, ent));
                s
            },
            &AllBut(ref ent) => {
                let mut s = self.token_universe.clone();
                s.remove(self.eval_entity(ctx, ent));
                s
            },
            &IsHuman => self.token_players.clone(),
            &IsProjectile => self.token_projectiles.clone(),
            &Empty => TokenSet::new(),
            &Universe => self.token_universe.clone(),
        }
    }

    fn eval_location(&mut self, ctx: &EventContext, location: &Location) -> Point {
        use magic::Location::*;
        match location {
            &AtEntity(ref ent) => {
                let e = self.eval_entity(ctx, ent);
                self.point_of(e).expect("UH OH")
            },
            &Midpoint(ref locs) => {
                Point::midpoint(
                    & (
                        locs.iter()
                        .map(|x| self.eval_location(ctx, x))
                        .collect::<Vec<_>>()
                    )
                ).unwrap_or(Point::NULL)
            },
            &Choose(ref locs) => {
                if let Some(x) = self.rng.choose(locs) {
                    self.eval_location(ctx, x)
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

#[derive(Clone, Eq, PartialEq)]
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



pub struct Player {
    health: u32,
    health_max: u32,
    mana: u32,
    mana_max: u32,
    buffs: HashMap<Buff, (u8, f32)>,
    velocity: Vector,
    spells: Vec<Spell>,
}

impl Player {
    pub fn new(health_max: u32, mana_max: u32) -> Player {
        Player {
            health_max: health_max,
            health: health_max,
            mana_max: mana_max,
            mana: mana_max,
            buffs: HashMap::new(),
            velocity: Vector::NULL,
            spells: Vec::new(),
        }
    }

    pub fn add_spell(&mut self, spell: Spell) {
        self.spells.push(spell);
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


pub fn game_loop() {
    let mut me = Player::new(100, 100);
    let mut rng = Isaac64Rng::new_unseeded();
    let mut spells = 0;
    while spells < 5 {
        let (spell, complexity) = generate::spell(4, &mut rng);
        if 5 <= complexity && complexity <= 32 {
            println!("spell (complexity: {})\n{:#?}\n\n", complexity, &spell);
            me.add_spell(spell);
            spells += 1;
        } else {
            println!("bad complexity of {}", complexity);
            println!("BAD:: {:#?}", &spell);
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
                space.add_velocity_to_player(token, Vector::new_from_directional(dir, 4.0));
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
        for (&tok, &(ref pt, ref player)) in space.players.iter() {
            image(&wiz_sprite.texture, c.transform
                .trans(
                    pt.0 as f64 - (wiz_sprite.center.0 as f64),
                    pt.1 as f64 - (wiz_sprite.center.1 as f64),
                ).zoom(0.3), g);
        }
    });
}