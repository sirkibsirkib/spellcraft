use buffs::*;


#[derive(Copy, Clone)]
pub struct ESlot(pub u8); // one entity per slot

#[derive(Copy, Clone)]
pub struct ESetSlot(pub u8); // one entity set per slot 

#[derive(Copy, Clone)]
pub struct DSlot(pub u8); // one discrete per slot

#[derive(Copy, Clone)]
pub struct LSlot(pub u8); // one Location per slot



#[derive(Clone, Debug)]
pub struct BuffStack(Buff, Discrete);

#[derive(Clone, Debug)]
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
    SpawnProjectileAt(Box<ProjectileBlueprint>, Location),
}

#[derive(Clone, Debug)]
pub enum Direction {
    TowardLocation(Location),
    ConstRad(f32),
    BetweenRad(f32, f32),
    Choose(Vec<Direction>),
    ChooseWithinRadOf(Box<Direction>, f32),
}

#[derive(Clone, Debug)]
pub enum Definition {
    ESet(ESetSlot, EntitySet),
    E(ESlot, Entity),
    D(DSlot, Discrete),
}

#[derive(Clone, Debug)]
pub enum Location {
    AtEntity(Entity),
    Midpoint(Vec<Location>),
    Choose(Vec<Location>),
    LoadLocation(LSlot),
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum EntitySet { // describes how to "subset" the universe
    Nand(Vec<EntitySet>),
    And(Vec<EntitySet>),
    Or(Vec<EntitySet>),
    Only(Entity),
    IsInSlot(ESetSlot),
    WithinRangeOf(Entity, Discrete),
    HasMinResource(Entity, Resource),
    EnemiesOf(Entity),
    AllBut(Entity),
    IsHuman,
    IsProjectile,
    Empty,
    Universe,
}

#[derive(Clone, Debug)]
pub enum EntitySetCmp {
    Nand(Vec<EntitySetCmp>),
    And(Vec<EntitySetCmp>),
    Or(Vec<EntitySetCmp>),

    Subset(EntitySet, EntitySet),
    Superset(EntitySet, EntitySet),
    Equal(EntitySet, EntitySet),
    Contains(EntitySet, Entity),
}

#[derive(Clone, Debug)]
pub enum Discrete {
    Const(i32),
    Range(i32, i32),
    WithinPercent(i32, f32),
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

#[derive(Clone, Debug)]
pub enum Resource {
    Mana(Discrete),
    Health(Discrete),
    BuffStacks(Buff, Discrete),
}

#[derive(Clone)]
pub enum Entity {
    LoadEntity(ESlot),
    FirstOf(Box<EntitySet>),
    Choose(Box<EntitySet>),
    ClosestFrom(Box<EntitySet>, Box<Location>),
    LastOf(Box<EntitySet>),
}

#[derive(Clone, Debug)]
pub struct Spell {
    pub on_cast: Vec<Instruction>, //    [][0:caster][]
    pub requires: Condition, //     [][0:caster][]
    pub consumes: Vec<Resource>, // [][0:caster][]
}

#[derive(Clone, Debug)]
pub struct ProjectileBlueprint {
    pub on_create: Instruction,
    pub on_collision: Instruction,
    pub collides_with: EntitySet,
    pub on_destroy: Instruction,
    pub lifetime: Discrete,
}

fn blink_projectile() -> ProjectileBlueprint {
    let create =
    Instruction::AddVelocity(
        Entity::LoadEntity(ESlot(0)),
        Direction::TowardLocation(
            Location::LoadLocation(LSlot(0))
        ),
        Discrete::Const(10),
    );
    let this = Entity::LoadEntity(ESlot(0));
    let here = Location::AtEntity(this.clone());
    let teleport = 
    Instruction::MoveEntity(
        //who
        Entity::ClosestFrom(
            Box::new(
                EntitySet::And(vec![
                    EntitySet::HasMinResource(
                        this.clone(),
                        Resource::BuffStacks(
                            Buff::Electrified,
                            Discrete::Const(1),
                        ),
                    ),
                    EntitySet::AllBut(this.clone()),
                ])
            ),
            Box::new(here.clone()),
        ),
        //where
        here,
    );
    ProjectileBlueprint {
        on_create: create,
        on_collision: Instruction::Destroy(this.clone()),
        collides_with: EntitySet::AllBut(this.clone()),
        on_destroy: teleport,
        lifetime: Discrete::Const(3),
    }
}

pub fn combat_blink() -> Spell {
    
    let enemies_within_10 = 
    Condition::MoreThan(
        Discrete::Cardinality(
            Box::new(
                EntitySet::WithinRangeOf(
                    Entity::LoadEntity(ESlot(0)),
                    Discrete::Const(10),
                ),
            )
        ),
        Discrete::Const(0),
    );
    let mana =
    Resource::Mana(
        Discrete::Sum(vec![
            Discrete::Const(30),
            Discrete::Mult(vec![
                Discrete::CountStacks(
                    Buff::Electrified,
                    Entity::LoadEntity(ESlot(0)),
                ),
                Discrete::Const(10),
            ])
        ])
    );
    let electrify =
    Instruction::AddResource(
        Entity::LoadEntity(ESlot(0)),
        Resource::BuffStacks(
            Buff::Electrified,
            Discrete::Const(1),
        ),
    );
    let shoot_projectile =
    Instruction::SpawnProjectileAt(
        Box::new(blink_projectile()),
        Location::AtEntity(
            Entity::LoadEntity(ESlot(0)),
        ),
    );
    Spell {
        on_cast: vec![electrify, shoot_projectile],
        requires: enemies_within_10,
        consumes: vec![mana],
    }
}

pub fn swarm() -> Spell {
    // distributes 20 stacks of Swarm amongst all enemies within X range of caster
    // where X is (4 + number of `Toxified` stacks on caster)
    let def_nearby =
    Instruction::Define(
        Definition::ESet(
            ESetSlot(0),
            EntitySet::And(vec![
                EntitySet::WithinRangeOf(
                    Entity::LoadEntity(ESlot(0)),
                    Discrete::Sum(vec! [
                        Discrete::Const(4),
                        Discrete::CountStacks(
                            Buff::Toxified,
                            Entity::LoadEntity(ESlot(0)),
                        ),
                    ]),
                ),
                EntitySet::EnemiesOf(
                    Entity::LoadEntity(ESlot(0)),
                ),
            ]),
        ),
    );
    let def_stacks =
    Instruction::Define(
        Definition::D(
            DSlot(0),
            Discrete::Div(
                Box::new(
                    Discrete::Const(20)
                ),
                Box::new(
                    Discrete::Cardinality(
                        Box::new(
                            EntitySet::IsInSlot(ESetSlot(0)),
                        ),
                    ),
                ),
            ),
        ),
    );
    let go = 
    Instruction::ForEachAs(
        ESlot(1),
        EntitySet::IsInSlot(
            ESetSlot(0)
        ),
        vec![
            Instruction::AddResource(
                Entity::LoadEntity(
                    ESlot(1)
                ),
                Resource::BuffStacks(
                    Buff::Swarm,
                    Discrete::LoadFrom(DSlot(0)),
                ),
            ),
        ],
    );
    Spell {
        on_cast: vec![def_nearby, def_stacks, go],
        requires: Condition::Top,
        consumes: vec![Resource::Mana(Discrete::Const(50))],
    }
}

