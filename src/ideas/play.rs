use std::collections::{HashMap,HashSet};

pub struct Player {
    pos: Point2D,
    health: u32,
    health_max: u32,
    mana: u32,
    mana_max: u32,
    buffs: H
}

pub struct Point2D(pub i32, pub i32);

