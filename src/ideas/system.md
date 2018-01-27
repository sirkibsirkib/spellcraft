Terms:
* __Entity__:
Some NPC friendly, NPC enemy or player. An entity can cast _spells_.

* __Actor__:
A generalization over things that have a compile-time fixed set of _event handlers_ and perform _effects_ in response to these events being called.

* __Effect__:
An effect is a tree-like data structure of nested enum variants. This structure is associated with an event handler. When the event occurs (in a certain context of _entity arguments_), the tree is traversed and parsed to apply changes to the world. For example, decreasing an entity's health or applying a _buff_.

* __Entity Predicate__:
An _effect_ is a tree with the structure of the branches and nodes describing which actions to perform when the _effect_ 'happens'. However, the world is changing, and thus the things the effect is happening to are described abstractly, as some predicate (eg: all entities with 2+ stacks of some _buff_ within 5 meters of a specific entity). At runtime, these predicates are translated to specific _entity sets_.

* __Entity Set__:
At runtime, an _entity predicate_ is instantiated as a specific _entity set_ of particular entities. 

* __Entity Arguments__:
_Effects_ occur in the context of a given _event handler_ being triggered. This handler usually logically occurs as a result of some number of entities (For example, a projectile colliding with a player involves two entities). The associated _effect_ must thus have a matching arity (the number of arguments) to match the number of entities in the context. For example, consider some effect `destroy` which has arity 1 (it destroys one specific entity). Some event handler for a projectile called `self_created` would likely require a 1-arity (AKA unary) effect. Here `destroy` would be logical, and the one argument from the context would be understood to be the projectile itself.
In this way, all event handers have a specific arity, and precisely define which each argument is associated with in the context.

* __Buff__:
The world has a large _fixed_ set of buffs, where a buff is associated with some temporary 'change' to an entity; For example: {`Tired`, `Hungry`, `Bleeding`}. They can be thought of as all the unique 'labels' for temporary changes to the state of an _entity_. Each buff maps to some influence on the entity afflicted with it. For intance: `Crippled` reduces the movement speed of an entity. _Buffs_ do not occur 'naked' attached to entities, but rather as a tuple with:
    1. The specific buff label eg. `Crippled`.
    1. Time remaining before it vanishes.
    1. _Buff Tier_
    1. _Buff Stacks_

* __Buff Tier__:
_Buffs_ that have intrinsic effects have _tiers_ which simply scale up the strength of their intrinsic effects. For example: A tier 2 `Confused` might decrease the entities' `mana` _resource_ by 20, while tier 1 would decrease it by only 10. If _buffs_ differ only by _tier_ they are considered different, and will not stack together.

* __Buff Stack__:
_Buffs_ occur along with a counter representing the size of the _stack_. This corresponds logically with the _multiplicity_ of the buff. By default, two stacks of the same buff will coalesce and combine their stack numbers. However, some buffs have different or unique stacking behaviours.


* __Spell__: A type of actor wef wef 
* __Resource__:



The world is filled with Entities. These are NPCs, AI enemies, players.
Each entity maintains its own `Buff` stacks.

Entities can know `Spell`s. These spells can be cast by the entity that knows
them. A spell can be thought of a small instruction set

A spell is ultimately a small instruction set of `Effect`s that are applied
to entities (or sets of entities) in which order in which way etc. However, the 
same spell is cast in various circumstances. As such, the spell is described in
a sort of _abstracted_ language that captures the nature of the environment.
Accordingly, a spell (under the hood) takes the form as a complex _term_, with
N-ary predicates indicating relationships between the entities involved in the 
effect(s).

Effects are assessed (seemingly) atomically, with their influence
on the world being carried out all at once. How does an effect
NOW influence another effect later?
* By using `Buff`s. A buff can be applied to an entity NOW, and checked LATER.
* By calling a _timer_. Some 
Whenever _time_ is involved with an effect, the