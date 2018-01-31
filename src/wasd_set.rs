

#[derive(Copy, Clone, Debug)]
pub struct WasdSet {
    w: Setting,
    a: Setting,
    s: Setting,
    d: Setting,
    antagonistic: bool,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Setting {
    Pressed, Disabled, Released, 
}

macro_rules! press {
    ($this:expr, $me:ident, $antagonist:ident) => {
        $this.$me = Setting::Pressed;
        if $this.$antagonist == Setting::Pressed {
            $this.$antagonist = Setting::Disabled;
        }
    };
}

macro_rules! release {
    ($this:expr, $me:ident, $antagonist:ident) => {
        $this.$me = Setting::Released;
        if $this.$antagonist == Setting::Disabled {
            $this.$antagonist = Setting::Pressed;
        }
    };
}

impl WasdSet {
    pub fn new(antagonistic: bool) -> WasdSet {
        use self::Setting::*;
        WasdSet {
            w:Released, a:Released, s:Released, d:Released, antagonistic: antagonistic,
        }
    }
    pub fn press_w(&mut self) { press![self, w, s]; }
    pub fn press_a(&mut self) { press![self, a, d]; }
    pub fn press_s(&mut self) { press![self, s, w]; }
    pub fn press_d(&mut self) { press![self, d, a]; }
    
    pub fn release_w(&mut self) { release![self, w, s]; }
    pub fn release_a(&mut self) { release![self, w, s]; }
    pub fn release_s(&mut self) { release![self, w, s]; }
    pub fn release_d(&mut self) { release![self, w, s]; }

    pub fn is_pressed_w(&self) -> bool { self.w == Setting::Pressed }
    pub fn is_pressed_a(&self) -> bool { self.a == Setting::Pressed }
    pub fn is_pressed_s(&self) -> bool { self.s == Setting::Pressed }
    pub fn is_pressed_d(&self) -> bool { self.d == Setting::Pressed }

    pub fn direction(&self) -> WasdDirection {
        // println!("{:?}\n\n", self);
        use self::Setting::*;
        use self::WasdDirection::*;
        if self.w == Pressed && self.s == Released {
            //W..
            if self.a == Pressed && self.d == Released {
                WA
            } else if self.d == Pressed && self.a == Released {
                WD
            } else { W }
        } else if self.s == Pressed && self.w == Released {
            //S..
            if self.a == Pressed && self.d == Released {
                SA
            } else if self.d == Pressed && self.a == Released  {
                SD
            } else { S }
        } else {
            //..
            if self.a == Pressed && self.d == Released {
                A
            } else if self.d == Pressed && self.a == Released  {
                D
            } else { None }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum WasdDirection {
    None,
    W, A, S, D,
    WA, WD, SA, SD,
}