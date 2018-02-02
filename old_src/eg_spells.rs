
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
        on_create: vec![create],
        on_collision: vec![Instruction::Destroy(this.clone())],
        collides_with: EntitySet::AllBut(this.clone()),
        on_destroy: vec![teleport],
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
        Rc::new(blink_projectile()),
        Location::AtEntity(
            Entity::LoadEntity(ESlot(0)),
        ),
    );
    Spell {
        on_cast: vec![electrify, shoot_projectile],
        requires: Box::new(enemies_within_10),
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
        requires: Box::new(Condition::Top),
        consumes: vec![Resource::Mana(Discrete::Const(50))],
    }
}