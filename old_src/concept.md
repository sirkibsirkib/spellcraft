# Spells
spells have:
    cast cooldown (c_cd)
        remaining cooldown set to this after a cast
    remaining cooldown (r_cd)
        counts down to 0 in real time. when it reaches 0, 
    max charges (m_ch)
        default is one
    chargex (ch)

        

# Buffs
have:
    duration
        time taken between being applied and removed
    application effect
        performs this modification to the entity when applied
    removal effect
        performs this modification to the entity when removed
        (usually opposite to application effect)
    refresh_idempotence
        boolean value. if TRUE, refresh does NOTHING except change cooldown
        if FALSE, refresh performs REMOVE + APPLY
    max_stacks
        if buff applied to someone with existing buff:
            if < max_stacks, the new buff stacks application.
            else: the new buff just refreshes old buff

# Projectiles
have:
    movement type (enum):
        float
            maintains linear velocity
        accelerate
            maintains velocity with += speed each tick
        ...?
    graphic
    collision candidates:
        a set of things that will cause collision event
    collision effects:
        pick from {immobilize, consume}
    expiry effects:    