


pub struct WasdSet {
    w: Setting,
    a: Setting,
    s: Setting,
    d: Setting,
    antagonistic: bool,
}

enum Setting {
    Pressed, Disabled, Released, 
}

macro_rules! press {
    ($self, $me:ident, $antagonist:ident) => {
        $self.$me = Setting::Pressed;
        if $self.antagonistic && $self.$antagonist == Setting::Pressed {
            $self.$antagonist == Setting::Disabled
        }
    };
}

macro_rules! release {
    ($self, $me:ident, $antagonist:ident) => {
        $self.$me = Setting::Released;
        if $self.$antagonist == Setting::Disabled {
            $self.$antagonist == Setting::Pressed
        }
    };
}

impl WasdSet {
    pub fn new(antagonistic: bool) -> WasdSet {
        use Setting::*;
        WasdSet {
            w:Released, a:Released, s:Released, d:Released,
        }
    }
    pub fn press_w(&mut self) { press![self, w, s] }
    pub fn press_a(&mut self) { press![self, a, d] }
    pub fn press_s(&mut self) { press![self, s, w] }
    pub fn press_d(&mut self) { press![self, d, a] }
    
    pub fn release_w(&mut self) { release![self, w, s] }
    pub fn release_a(&mut self) { release![self, w, s] }
    pub fn release_s(&mut self) { release![self, w, s] }
    pub fn release_d(&mut self) { release![self, w, s] }

    pub fn is_pressed_w(&self) -> bool { self.w == Setting::Pressed }
    pub fn is_pressed_a(&self) -> bool { self.a == Setting::Pressed }
    pub fn is_pressed_s(&self) -> bool { self.s == Setting::Pressed }
    pub fn is_pressed_d(&self) -> bool { self.d == Setting::Pressed }

    pub fn direction(&self) -> WasdDirection {
        use WasdDirection::*;
        if self.w == Setting::Pressed && self.s != Setting::Pressed {
            if self.a == Setting::Pressed && self.d != Setting::Pressed {
                WA
            } else if self.d == Setting::Pressed {
                WD
            } else { W }
        } else if self.s == Setting::Pressed {
            if self.a == Setting::Pressed && self.d != Setting::Pressed {
                SA
            } else if self.d == Setting::Pressed {
                SD
            } else { S }
        } else {
            if self.a == Setting::Pressed && self.d != Setting::Pressed {
                A
            } else if self.d == Setting::Pressed {
                D
            } else { None }
        }
    }
}

pub enum WasdDirection {
    None,
    W, A, S, D,
    WA, WD, SA, SD,
}