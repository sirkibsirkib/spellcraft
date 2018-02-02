## The Magic System

Here I describe a system for creating datastructures that represent behaviours of _spells_ and related effects in a typical rpg-like game. I refer to these nested datastructures as `magic`. The intention is to be able to be able to build all spells always out of the same magical components. Magic must be powerful enough to be able to describe complex behaviours and interesting interactions, yet simple enough that it can remain general and be used by all spells without introducing too much clutter.

The main src for magic can be found at [/src/magic.rs](https://github.com/sirkibsirkib/spellcraft/blob/master/src/magic.rs)

An arbitrary spell is defined by the structure `Spell`. Spells can also temporarily manifest a `Projectile` in the world, which can collide with players and such. Things in the game that have positions in a shared coordinate space (only players and projectiles) are referred to under the umbrella term _entities_.

A spell is a structure with a predefined set of fields at the root level. All spells have the same set of fields, but vary the _magical_ contents of the fields. This is the bridge between what is hard-coded and what is not: The runtime engine will expect and parse a given spell _field_ in reponse to a gameplay event (eg: player X casts spell Y) or under certain conditions (eg: the engine will determine whether projectiles X and Y collide or pass through one another).

As the magic does not change over time, magic data is defined in _abstract_ language, while the data is parsed in specific _concrete_ cases. The interpretation of the mapping from abstract to concrete is done by means of interactions with the _context_ in which a field's magic is parsed. This 'context' essentially stores mappings from abstract tokens eg: `entity(0)` to concrete tokens `identifier::player(374)`. The spell data is formulated to interact with this context, 'loading' from the context to make an abstract token concrete, and 'defining' something into the context, to represent a concrete concept to be loaded by an abstraction in another part of the data.

### Example

```Rust
/*
Pseudo-magic for an `Instruction` to teleport
the caster to the closest other player
*/
MoveEntity(
    LoadEntity(ESlot(0))),
    AtEntity(
        ClosestFrom(And(
            IsHuman,
            AllBut(ESlot(0)),
        )
    ),
)
```

## Magic Over Time

Magic is evaluated _all at once_, and context does NOT persist accross multiple events. How then would one create a spell that needs to capture a notion of time? Doing something after a delay or under a condition to-be-determined?
1. __Projectiles__

    Spells can create a projectile, which at a later stage performs its OWN events. The combination of these two events result in delayed action.

1. __Buffs__

    Events can reason over and manipulate the buffs of entities, but the buffs themselves have a sense of time. For instance, a spell might apply a damaging bleed effect on all enemies in a radius on cast.

The use of projectiles and buffs also impose some unavoidable restrictions which are rather desirable: Events later can _change_ the behaviour of magic down the line. Projectiles can collide or be prematurely destroyed. Buffs can be removed or multiplied. This means that the spells of all players _interact_.


## Terms

* __Entity__

    Some player's avatar or a projectile. Any object that occupies the world's coordinate plain, `tick()`s with the progress of time alongside other entities and potentially collides. Players can cast _spells_. Entities possess _resources_.

* __Magic__

    Magic refers to various typed intertwined enum classes whose variants potentially contain nested magic. When parsed along with a _context_, the abstract terms inside the magic map to concrete things inside the world at runtime, such as _entities_. _Spells_ and projectiles have their behaviours entirely defined by magic, and thus can be changed if their magic is changed.


* __Spell__

    A spell is a datastructure with a defined set of _fields_. Each field contains _magic_. At runtime, the engine will parse fields in response to certain in-game events. At a high-level, the spell data structure contains an abstracted instruction set of how the spell should behave under various conditions.

* __Resource__

    Entities possess resources such as mana and health, along with _buff stacks_. 'Resource' can be used to refer either to the abstract resource magic enum OR just some bespoke data contained within player objects etc.

* __Token__

    A token exists at runtime, and is used to track individual game objects, namely entities. Tokens provide the means of locating and storing game data. As such, they provide concrete representations of specific entities.

* __Entity Set__

    Some magic enums reason over sets of abstract entities. For example: All players. Essentially, an entity set describes some predicate over entities in the game universe. Along with a context, such a set can be made concrete into a _token set_.

* __Token Set__

    Token sets are the result of making an entity set conrete.

* __Buff__

    The world has a large _fixed_ set of buffs, where a buff is associated with some temporary 'change' to an entity; For example: {`Tired`, `Hungry`, `Bleeding`}. They can be thought of as all the unique 'labels' for temporary changes to the state of an _entity_. Each buff maps to some influence on the entity afflicted with it. For intance: `Crippled` reduces the movement speed of an entity. _Buffs_ do not occur 'naked' attached to entities, but rather as a tuple with:
    1. The specific buff label eg. `Crippled`.
    1. Time remaining before it vanishes.
    1. _Buff Tier_
    1. _Buff Stacks_

* __Buff Tier__

    _Buffs_ that have intrinsic effects have _tiers_ which simply scale up the strength of their intrinsic effects. For example: A tier 2 `Confused` might decrease the entities' `mana` _resource_ by 20, while tier 1 would decrease it by only 10. If _buffs_ differ only by _tier_ they are considered different, and will not stack together.

* __Buff Stack__

    _Buffs_ occur along with a counter representing the size of the _stack_. This corresponds logically with the _multiplicity_ of the buff. By default, two stacks of the same buff will coalesce and combine their stack numbers. However, some buffs have different or unique stacking behaviours.

