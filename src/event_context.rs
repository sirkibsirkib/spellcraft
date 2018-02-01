use magic::*;
use std::collections::HashMap;
use play::{Token, TokenSet};
use movement_2d::*;

#[derive(Clone, Debug)]
pub struct EventContext {
    pub e: HashMap<ESlot, Token>,
    pub e_set: HashMap<ESetSlot, TokenSet>,
    pub l: HashMap<LSlot, Point>,
    pub d: HashMap<DSlot, i32>,
}
impl EventContext {
    pub fn new() -> EventContext {
        EventContext {
            e: HashMap::new(),
            e_set: HashMap::new(),
            l: HashMap::new(),
            d: HashMap::new(),
        }
    }
}
impl ContextFor<ESlot, Token> for EventContext {
    fn define(&mut self, k:ESlot, v:Token) { self.e.insert(k, v); }
    fn load(&self, k:&ESlot) -> Option<&Token> { self.e.get(k) }
}
impl ContextFor<ESetSlot, TokenSet> for EventContext {
    fn define(&mut self, k:ESetSlot, v:TokenSet) { self.e_set.insert(k, v); }
    fn load(&self, k:&ESetSlot) -> Option<&TokenSet> { self.e_set.get(k) }
}
impl ContextFor<LSlot, Point> for EventContext {
    fn define(&mut self, k:LSlot, v:Point) { self.l.insert(k, v); }
    fn load(&self, k:&LSlot) -> Option<&Point> { self.l.get(k) }
}
impl ContextFor<DSlot, i32> for EventContext {
    fn define(&mut self, k:DSlot, v:i32) { self.d.insert(k, v); }
    fn load(&self, k:&DSlot) -> Option<&i32> { self.d.get(k) }
}


pub trait ContextFor<K,V> {
    fn define(&mut self, k:K, v:V);
    fn load(&self, k:&K) -> Option<&V>;
}