use buffs::*;
use std::rc::Rc;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ESlot(pub u8); // one entity per slot

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ESetSlot(pub u8); // one entity set per slot 

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct DSlot(pub u8); // one discrete per slot

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LSlot(pub u8); // one Location per slot

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BuffStack(Buff, Discrete);

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Instruction {
    Define(Definition),
    ITE(Condition, Vec<Instruction>, Vec<Instruction>),
    CallWith(Definition, Vec<Instruction>), //necessary?
    ForEachAs(ESlot, EntitySet, Vec<Instruction>),
    DestroyWithoutEvent(Entity),
    Destroy(Entity),
    MoveEntity(Entity, Location),
    AddResource(Entity, Resource),
    AddVelocity(Entity, Direction, Discrete), // last arg is "speed"
    SpawnProjectileAt(Rc<ProjectileBlueprint>, Location),
    Nothing,
}

#[derive(Clone, PartialEq, PartialOrd, Copy)]
pub struct F32(pub f32);
impl Eq for F32 {}


#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    TowardLocation(Location, Location),
    ConstRad(F32),
    BetweenRad(F32, F32),
    Choose(Vec<Direction>),
    ChooseWithinRadOf(Box<Direction>, F32),
}

#[derive(Clone, Eq, PartialEq)]
pub enum Definition {
    ESet(ESetSlot, EntitySet),
    E(ESlot, Entity),
    D(DSlot, Discrete),
    L(LSlot, Location),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Location {
    AtEntity(Entity),
    Midpoint(Vec<Location>),
    Choose(Vec<Location>),
    LoadLocation(LSlot),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Condition {
    Nand(Vec<Condition>),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Top,
    Bottom,
    Equals(Discrete, Discrete),
    LessThan(Discrete, Discrete),
    MoreThan(Discrete, Discrete),
    EntitySetCmp(EntitySetCmp),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EntitySet { // describes how to "subset" the universe
    None(Vec<EntitySet>),
    And(Vec<EntitySet>),
    Or(Vec<EntitySet>),
    Only(Entity),
    IsInSlot(ESetSlot),
    WithinRangeOf(Entity, Discrete),
    HasMinResource(Resource),
    EnemiesOf(Entity),
    AllBut(Entity),
    IsHuman,
    IsProjectile,
    Empty,
    Universe,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum EntitySetCmp {
    Nand(Vec<EntitySetCmp>),
    And(Vec<EntitySetCmp>),
    Or(Vec<EntitySetCmp>),

    Subset(EntitySet, EntitySet),
    Superset(EntitySet, EntitySet),
    Equal(EntitySet, EntitySet),
    Contains(EntitySet, Entity),
}

#[derive(Clone, Eq, PartialEq)]
pub enum Discrete {
    Const(i32),
    Range(i32, i32),
    WithinPercent(i32, F32),
    Div(Box<Discrete>, Box<Discrete>),
    Sum(Vec<Discrete>),
    Neg(Box<Discrete>),
    Mult(Vec<Discrete>),
    Max(Vec<Discrete>),
    Min(Vec<Discrete>),
    CountStacks(Buff, Entity),
    CountDur(Buff, Entity),
    Choose(Vec<Discrete>),
    Cardinality(Box<EntitySet>),
    LoadFrom(DSlot),
}
impl Discrete {
    pub fn estimate(&self) -> f32 {
        use self::Discrete::*;
        match self {
            &Const(x) => x as f32,
            &Range(x, y) => (x + y) as f32 * 0.5,
            &WithinPercent(x, _) => x as f32,
            &Div(ref a, ref b) => a.estimate() / {
                let z = b.estimate();
                if z == 0.0 {0.000001} else {z}
            },
            &Sum(ref x) => x.iter().map(|x| x.estimate()).sum(),
            &Neg(ref x) => -x.estimate(),
            &Mult(ref x) => x.iter().fold(1.0, |x,y| x*y.estimate()),
            &Max(ref x) => {
                x.iter().fold(
                    ::std::f32::MIN,
                    |x,y| {let y = y.estimate(); if x < y {x} else {y}},
                )  
            },
            &Min(ref x) => {
                x.iter().fold(
                    ::std::f32::MAX,
                    |x,y| {let y = y.estimate(); if x > y {x} else {y}},
                )  
            },
            &CountStacks(_, _) => 2.0,
            &CountDur(_, _) => 20.0,
            &Choose(ref x) => {
                let mut tot = 0.0;
                let mut cnt = 0;
                for z in x {
                    tot += z.estimate();
                    cnt += 1;
                }
                tot / (cnt as f32)
            },
            &Cardinality(_) => 6.0,
            &LoadFrom(_) => 10.0,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Resource {
    Mana(Discrete),
    Health(Discrete),
    BuffStacks(Buff, Discrete),
}

#[derive(Clone, Eq, PartialEq)]
pub enum Entity {
    LoadEntity(ESlot),
    FirstOf(Box<EntitySet>),
    Choose(Box<EntitySet>),
    ClosestFrom(Box<EntitySet>, Box<Location>),
}

/////////////////////////////////////////////////////////////////////////

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Spell {
    pub on_cast: Vec<Instruction>, //    ent0:caster, loc0:cursor    
    pub requires: Box<Condition>, //     ent0:caster, loc0:cursor
    pub on_cooldown: Vec<Instruction>, //ent0:caster,
    pub consumes: Vec<Resource>, //      ent0:caster, loc0:cursor
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ProjectileBlueprint {
    pub on_create: Vec<Instruction>, //     ent0:caster, ent1:me, loc0:cursor    
    pub on_collision: Vec<Instruction>, //  ent0:caster, ent1:me, loc0:cursor    
    pub collides_with: EntitySet, //        ent0:caster, ent1:me, loc0:cursor    
    pub on_destroy: Vec<Instruction>, //    ent0:caster, ent1:me, loc0:cursor    
    pub lifetime: Discrete, //              ent0:caster, ent1:me, loc0:cursor    
}