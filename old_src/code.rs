use std::collections::HashMap;

pub struct Spell {
    caster_require: Entity,
    caster_consume: Vec<Resource>, // all in CONSUMES implicity also in `requires_caster_matches`
    on_cast: T1Effect, // arg0: self
}

pub struct ProjectileBlueprint {
    on_spawn: T1Effect, //arg0: self
    on_collide: T2Effect, //arg0: self, arg1: collided_with
    on_destroy: T1Effect, //arg0: self
    on_timer0: T1Effect, //arg0: self.  other events can SET the timer
}

pub enum T1Effect { // effect which takes two entities from context
    All(Vec<T1Effect>), // perform all with same args, independently
    Any(Vec<T1Effect>), // perform ONE of set
    ITE(Entity, Vec<T1Effect>, Vec<T2Effect>), //checks arg0. recurses with arg0
    EmitProjectile(Box<ProjectileBlueprint>),
    AddResource(Resource),
    MoveTo(Place),
    AddFirstArg(Box<T2Effect>, Entity),
    AddSecondArg(Box<T2Effect>, Entity),
    SwapArgs(Box<T1Effect>),
    AddHealth(Discrete),
    DupArg(Box<T2Effect>),
    AddBuffStacks(Buff, Discrete, Discrete), //buff, stacks, duration 
    ReplaceArg(Entity, Box<T1Effect>),
    Destroy,
    Nothing,
}

pub enum Place {
    OfEntity,
}

pub enum T2Effect { // effect which takes two entities from context
    All(Vec<T2Effect>), // perform all with same args, independently
    Any(Vec<T2Effect>), // perform ONE of set
    IArg0TE(Entity, Vec<T1Effect>, Vec<T1Effect>), // checks arg0. recurses with only arg1

    SplitArgs(T1Effect, T1Effect), // sugar for All(TakeFirstArg(...), TakeSecondArg(...))
    SwapPositions(Entity, Entity),
    SwapArgs(Box<T2Effect>),
    TakeFirstArg(T1Effect), // drops 2nd arg from context
    TakeSecondArg(T1Effect), // drops 1st arg from context
    Nothing,
}

// a predicate. when applied to an entity returns T or F. used to capture
// entity sets from the environment
pub enum Entity {
    // PickN(Box<Entity>, Discrete),
    And(Vec<Entity>),
    Or(Vec<Entity>),
    Nand(Vec<Entity>), //doubles as Not

    Argument,
    IsTheCaster,
    IsAPlayer,
    IsProjectile,
    HasResourceMin(Resource),
    HasResourceMax(Resource),
    HasResourceExact(Resource),
    HasWithinRange(Discrete, Box<Entity>),
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
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

pub enum Discrete {
    Exact(i32),
    OnInterval(i32, i32), //on interval [5,7] == OnInterval(5,7)
    UniformPercWithin(i32, f32), // within 5% of 10 == UniformPercWithin(10, 0.05)
}






pub fn fireball() -> Spell {
    let fireball = ProjectileBlueprint {
        on_spawn: T1Effect::Nothing, //arg0: self
        on_collide: T2Effect::SplitArgs(
            T1Effect::Destroy, // destroy the projectile
            T1Effect::AddHealth(Discrete::OnInterval(-50,-40)), //damage collidee
        ),
        on_destroy: T1Effect::ReplaceArg( //throw away projectile arg
            Entity::IsTheCaster,
            Box::new(
                T1Effect::AddBuffStacks(
                    Buff::Scalded,
                    Discrete::Exact(1), //1 stack of scalded
                    Discrete::Exact(3), //scald for 3 sec
                )
            ),
        ),
        on_timer0: T1Effect::Nothing, //arg0: self.  other events can SET the timer
    };
    Spell {
        caster_require: Entity::HasResourceExact(
            Resource::BuffStacks(Buff::Scalded, Discrete::Exact(0))
        ),
        caster_consume: vec![
            Resource::Mana(Discrete::Exact(50))
        ],
        on_cast: T1Effect::EmitProjectile(Box::new(fireball)),
    }
}