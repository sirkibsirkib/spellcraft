1. Figure out what patterns, constraints make a good spell
    * Within `Vec<Instruction>`, have definitions at the beginning.
    * Actually use defs
    * 

1. Figure out how to automatically NAME a spell
    * Crawl the structure, call into the Counter object to record stuff
    * associate fragments with certain behaviours
        1. syllable for the biggest spell cost
        1. extra word on the end for each projectile
        1. some fragments for noteworthy buffs 


1. Figure out how to display the details of a spell
    * A list of sentences. Each sentence

```
moves the caster to itself, pushes the caster toward -0.32 with 15 force.
cannot be cast.
applies approximately 31 stacks of Cold,
consumes 40 mana.
```