use std::collections::HashMap;

pub struct Spell {
    requires: Condition,
    consumes: Vec<Resource>,
    on_cast: Effect,
}

pub enum Buff {
    Poisoned, Envenomed, Bleed, Bandaged, Scalded, Tired, Energized, Asleep, 
    Frenzied, Rended, Panic, Crippled, Informed, Weakened,
    Empowered, Practiced, Familiar,
}

//describes a lower bound for some resources on a target
pub enum Resource {
    BuffStacks(Buff, Discrete),
    And(Vec<Resource>),
    Or(Vec<Resource>),
    Mana(Discrete),
    Health(Discrete),
}

//predicate describing some arbitrary condition of the caster
pub enum Condition {
    Resource(Resource),
    NearbyEntity(Entity, Discrete),
}

//predicate describing some entity
pub enum Entity {
    And(Vec<Entity>),
    Or(Vec<Entity>),
    Caster,
    Friendly,
    Enemy,
}

type Multiplied<T> = (u32, T);

pub enum Effect {
    Nothing,
    ITE(Condition, Vec<Effect>, Vec<Effect>),
    IT(Condition, Vec<Effect>),
    All(Vec<Effect>),
    Any(Vec<Effect>),

    WeightedAny(Vec<Multiplied<Effect>>),
    EffectOnCaster(Box<EffectOnEntity>),
    EmitProjectile(Box<ProjectileBlueprint>),
}

pub enum EffectOnEntity {
    Nothing,
    ITE(Condition, Vec<EffectOnEntity>, Vec<EffectOnEntity>),
    IT(Condition, Vec<EffectOnEntity>),
    All(Vec<EffectOnEntity>),
    Any(Vec<EffectOnEntity>),
    WeightedAny(Vec<Multiplied<EffectOnEntity>>),

    Effect(Effect),
    AddResource(Resource),
    RemoveResource(Resource),
}

pub struct ProjectileBlueprint {
    on_spawn: Effect,
    on_collide: EffectOnEntity,
    on_destroy: Effect,
    destroy_when_any: Vec<DestroyWhen>,
    look: Visual,
    movement_initial: MovementInitial,
    movement_change: Vec<MovementChange>,
}

pub enum DestroyWhen {
    TickCondition(Condition),
    ColliderIs(Entity),
    AfterTime(Discrete),
    //IfVarMin(u32), If local_var >= X
}

pub enum Discrete {
    Exact(i32),
    OnInterval(i32, i32), //on interval [5,7] == OnInterval(5,7)
    UniformPercWithin(i32, f32), // within 5% of 10 == UniformPercWithin(10, 0.05)
}

pub enum MovementInitial {
    TowardCursor,
    Random,
    GivenDirection(f32),
    // TowardEntity(Entity),
}

pub enum MovementChange {
    CurveLeft(f32),
    CurveRight(f32),
    AddVelocity(f32),
    MultVelocity(f32),
}

pub struct ProjectileInstance {
    bp: ProjectileBlueprint,
    dir: f32,
    spe: f32,
    coord: Coord,
}

enum Visual {
    Iceball, Fireball, Skullball,
}

struct BuffStack(u32, u32); // (stacks, sec_remaining)

struct Player {
    coord: Coord,
    spells: Vec<Spell>,
    buffs: HashMap<Buff, Vec<BuffStack>>,
}

struct Coord(f32, f32);

pub fn fireball() -> Spell {
    let fireball = ProjectileBlueprint {
        on_spawn: Effect::Nothing,
        on_collide: EffectOnEntity::RemoveResource(
            Resource::Mana(Discrete::Exact(50))
        ),
        on_destroy: Effect::Nothing,
        destroy_when_any: vec![DestroyWhen::ColliderIs(Entity::Enemy)],
        look: Visual::Fireball,
        movement_initial: MovementInitial::TowardCursor,
        movement_change: vec![MovementChange::MultVelocity(0.9)],
    };
    Spell {
        requires: Condition::Resource(
            Resource::BuffStacks(Buff::Scalded, Discrete::Exact(0))
        ),
        consumes: vec![
            Resource::Mana(Discrete::Exact(50))
        ],
        on_cast: Effect::EmitProjectile(Box::new(fireball)),
    }
}