pub struct ESlot(pub u8); // one entity per slot
pub struct ESetSlot(pub u8); // one entity set per slot 
pub struct DSlot(pub u8); // one discrete per slot

pub enum Buff {
    Swarm, Burned, Cold, Chilled, Toxified, Envenomed
}

pub Struct BuffStack(Buff, Discrete);

pub enum Instruction {
    // you ONLY get the context from the layers above/outside.
    // the only way to extract from context is using EntitySet::WithinSlot(x) // `gets` from slot

    Define(Definition),
    //NOTE THESE THREE ARE IN DIFFERENT SLOTINDEX SPACES

    ITE(Condition, Vec<Instruction>, Vec<Instruction>),
    CallWith(Definition, Vec<Instruction>), //necessary?
    ForEachAs(SlotIndex, EntitySet, Vec<Instruction>),

    Destroy(Entity),
    AddResource(Entity, Resource),
    SpawnProjectileAt(ProjectileBlueprint, Location),
}

pub enum Definition {
    ESet(ESetSlot, EntitySet),
    E(ESlot, Entity),
    D(DSlot, Discrete),
}

pub enum Location {
    AtEntity(Entity),
    Midpoint(Vec<Location>),
    Choose(Vec<Location>),
}

pub struct ProjectileBlueprint {

}

////

pub enum Condition {
    Nand(Vec<Condition>),
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Equals(Discrete),
    LessThan(Discrete),
    MoreThan(Discrete),
    EntitySetCmp(EntitySetCmp),
}

pub enum EntitySet { // describes how to "subset" the universe
    Nand(Vec<EntitySet>),
    And(Vec<EntitySet>),
    Or(Vec<EntitySet>),
    Only(Entity),
    IsInSlot(ESetSlot),
    WithinRangeOf(Entity, Discrete),
    HasMinResource(Resource),
    EnemiesOf(Entity),
    IsHuman,
    IsProjectile,
}

pub enum EntitySetCmp {
    Nand(Vec<EntitySetCmp>),
    And(Vec<EntitySetCmp>),
    Or(Vec<EntitySetCmp>),

    Subset(EntitySet, EntitySet),
    Superset(EntitySet, EntitySet),
    Equal(EntitySet, EntitySet),
    Contains(EntitySet, Entity),
}

pub enum Discrete {
    Exact(i32),
    Range(i32, i32),
    Div(Box<Discrete>, Box<Discrete>),
    Sum(Vec<Discrete>),
    Neg(Box<Discrete>),
    Mult(Vec<Discrete>),
    Max(Vec<Discrete>),
    Min(Vec<Discrete>),
    CountStacks(Buff, Entity),
    CountDur(Buff, Entity),
    Choose(Vec<Discrete>),
    WithinPercent(i32, f32),
    Cardinality(EntitySet),
    LoadFrom(DSlot),
}

pub enum Resource {
    Mana(u32),
    Health(u32),
    BuffStacks(u32),
}

pub enum Entity {
    LoadEntity(ESlot),
    FirstOf(EntitySet),
    Choose(EntitySet),
    LastOf(EntitySet),
}

pub struct Spell {
    on_cast: Instruction,
    requires: Vec<Resource>,
    consumes: Resource
}


pub fn swarm() -> {
    // distributes 20 stacks of Swarm amongst all enemies within X range of caster
    // where X is (4 + number of `Toxified` stacks on caster)

    //note 
    Spell {
        on_cast: vec![
            Instruction::Define(
                Definition::ESet(
                    ESetSlot(0),
                    EntitySet::And(vec![
                        EntitySet::WithinRangeOf(
                            Entity::LoadEntity(0),
                            Discrete::Sum(vec! [
                                Discrete::Exact(4),
                                Discrete::CountStacks(
                                    Buff::Toxified,
                                    Entity::LoadEntity(0),
                                ),
                            ]),
                        ),
                        EntitySet::EnemiesOf(
                            Entity::LoadEntity(0),
                        ),
                    ]),
                ),
            ),
            Instruction::Define(
                Definition::D(
                    DSlot(0),
                    Discrete::Div(
                        Box::new(
                            Discrete::Exact(20)
                        ),
                        Box::new(
                            Discrete::Cardinality(
                                EntitySet::IsInSlot(
                                    ESetSlot(0)
                                )
                            ),
                        ),
                    ),
                ),
            ),
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
                            Discrete::LoadFrom(
                                DSlot(0)
                            ),
                        ),
                    ),
                ]
            ),
        ],
    }
}

