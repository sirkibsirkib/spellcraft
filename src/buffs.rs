#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Buff {
    // ASSORTED
    Swarm,      // surrounded by biting insects. taking damage over time
    Mute,       // can't cast spells
    Wet,        // electric damage taken increased
    Stealth,    // 

    // PSYCHOLOGICAL BUFF
    Wary,           // applies `Tired` when removed. reduces projectile damage
    Wise,           // increased max mana
    Unpredictable,  // Cancels stacks with any other buff
    Resolute,       // forces applied dampened. `Tired` has no effect.
    Calm,           // Panic will not apply tired. Movement increased if below 100%
    Steady,         // removes Panicked if it has < stacks
    Panicked,       // movement speed increased. applies `Tired` when removed

    // PSYCHOLOGICAL DEBUFF
    Dizzy,      // spell positions randomized slightly
    Tired,      // movement slowed. ticks down ONLY when not moving
    Confused,   // spell cast effects INSTEAD remove one stack of confused
    Drained,    // Max mana reduced by 30
    Weary,      // spell casts take 20% more time

    // PHYSICAL DEBUFF
    Bleeding,   // takes DoT. every 5 stacks of Bleeding are replaced with 1 stack of hemorrhage.
    Hemorrhage, // take DoT. maintains equivalent Bleed stacks. Cap is 3.
    Bruised,    // Tough.S + Bruised.S <= 5. 
    Limping,    // movement controls become intermittently slowed
    Delicate,   // increased DoT taken

    // PHYSICAL BUFF
    Tough,      // damage taken > 20% of health is reduced by 10% 

    Hot,
    Burning,    // DoT
    Scalded,    // Takes 
    Warm,       // cancels COLD buffs

    Cold,
    Shivering,  // Periodically interrupts spellcasting
    Chilled,    // Movement speed slowed
    Cool,       // Cancels HOT buffs

    // Chemical Debuffs
    Electrified, 
    Toxified, 
    Poisoned, 
    Envenomed, 

    // Chemical Buffs
    Refreshed, // cancels drained
    Energized, // cancels tired. 
}




pub enum StackingBehaviour {
    Max, // duration is max(new_dur, old_dur)
    Min, // duration is min(new_dur, old_dur)
    IfMax, // if new_dur >= old_dur,
           // duration is old_dur and stacks = sum(old_stacks, new_stacks)
    Replace, // replace duration and stacks entirely.
}

pub fn stacking_method(buff: Buff) -> StackingBehaviour {
    use self::StackingBehaviour::*;
    use self::Buff::*;
    match buff {
        Toxified => Min,
        Envenomed => IfMax,
        Electrified => Replace,
        _ =>  Max
    }
}