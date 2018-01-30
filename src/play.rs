use ::std::collections::{HashMap,HashSet};
use magic::*;
use buffs::*;

pub struct Player {
    health: u32,
    health_max: u32,
    mana: u32,
    mana_max: u32,
    buffs: HashMap<Buff, (u8, u32)>,
}

pub struct Grid {
    players: HashMap<Point2D, Player>,
    // find_players: HashMap // TODO
}

impl Player {
    pub fn new(health_max: u32, mana_max: u32) -> Player {
        Player {
            health_max: health_max,
            health: health_max,
            mana_max: mana_max,
            mana: mana_max,
            buffs: HashMap::new(),
        }
    }

    pub fn apply_stacks(&mut self, buff: Buff, stacks: u8, duration: u32) {
        assert!(stacks > 0);
        assert!(duration > 0);
        if let Some(&mut (ref mut old_stacks, ref mut old_duration)) = self.buffs.get_mut(&buff) {
            use buffs::StackingBehaviour::*;
            match stacking_method(buff) {
                Min => {
                    *old_stacks += stacks;
                    if duration < *old_duration {
                        *old_duration = duration;
                    }
                },
                Max => {
                    *old_stacks += stacks;
                    if duration > *old_duration {
                        *old_duration = duration;
                    }
                },
                Replace => {
                    *old_stacks += stacks;
                    *old_duration = duration;
                },
                IfMax => {
                    if duration >= *old_duration {
                        *old_stacks += stacks;
                    }
                },
            }
        }
    }

    pub fn decrement_times(&mut self) {
        for buff in self.buffs.keys().map(|x| *x).collect::<Vec<_>>() {
            let mut remove = false;
            if let Some(&mut (_, ref mut old_duration)) = self.buffs.get_mut(&buff) {
                *old_duration -= 1;
                if *old_duration == 0 {
                    remove = true;
                } 
            }
            if remove {
                self.buffs.remove(&buff);
            }
        }
    }
}


pub struct Point2D(pub i32, pub i32);

