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