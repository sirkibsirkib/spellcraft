For the following, `E` represents the entity on which the buff is applied. `T` represents the tier of the buff, and `S` represents the stacks of the buff. 

*EXCESS*. When a buff has a limit of N and some added stacks would result in N+M, this incurs EXCESS M

Other variables are created on the fly.

Unless stated otherwise:
S caps out at 5.

1. __Bleed__: {dot, bleed}
Does X damage over time. Multiplied by S and
T. Excess removes bleed entirely and applies 1 stack of _Haemorrage_.

1. __Haemorrage__: {dot, bleed}
Does X damage over time. Does double if the target is moving. S max is 3. With each tick applies `Bleed` of the same tier until Bleed.S >= Haemorrage.S

1. __Confused__: {}
Intelligence of E is reduced by X.

1. __Dazed__: {}
Effects toward the cursor of E are nudged arbitrarily by a function of T and S

1. __Poisoned__: {dot, toxin}
When the cooldown reaches 1 sec, the buff is removed and replaced with _Toxified_ with the same S and T.

1. __Toxified__: {dot, toxic}
periodically does X*S damage and S -= 1.

1. __Envenomed__: {dot, toxin}
periodically applies _Toxified_ with same S and T.

1. __Frozen__: {cold}
cannot move or cast spells

1. __Warm__: {warm}
periodically removes stacks from self AND all {cold} buffs applied. Excess stacks apply 1 _Burned_ if burnedd == 0

1. __Cold__: {cold}
periodically removes stacks from self AND all {warm} buffs applied. Excess stacks apply 1 _Chilled_ if chilled == 0

1. __Chilled__: {cold}
movement speed is reduced by T*X*S

1. __Burned__: {warm}
damage taken above X has Y added (small). 

