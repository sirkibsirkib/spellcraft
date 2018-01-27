mod code;

fn main() {
    let fireball = code::fireball();
    println!("{:?}", std::mem::size_of_val(&fireball));

    println!("Spell {:?}", std::mem::size_of::<code::Spell>());
    println!("Condition {:?}", std::mem::size_of::<code::Condition>());
    println!("Resource {:?}", std::mem::size_of::<code::Resource>());
    println!("Effect {:?}", std::mem::size_of::<code::Effect>());
    println!("ProjectileBlueprint {:?}", std::mem::size_of::<code::ProjectileBlueprint>());
    println!("ProjectileInstance {:?}", std::mem::size_of::<code::ProjectileInstance>());
    println!("DestroyWhen {:?}", std::mem::size_of::<code::DestroyWhen>());
    println!("Discrete {:?}", std::mem::size_of::<code::Discrete>());
    println!("Entity {:?}", std::mem::size_of::<code::Entity>());
    println!("MovementInitial {:?}", std::mem::size_of::<code::MovementInitial>());
}
