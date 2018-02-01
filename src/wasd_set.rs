

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
    pub fn release_a(&mut self) { release![self, a, d]; }
    pub fn release_s(&mut self) { release![self, s, w]; }
    pub fn release_d(&mut self) { release![self, d, a]; }

    pub fn is_pressed_w(&self) -> bool {
        use self::Setting::*;
        self.w == Pressed && self.s != Disabled
    }
    pub fn is_pressed_a(&self) -> bool {
        use self::Setting::*;
        self.a == Pressed && self.d != Disabled
    }
    pub fn is_pressed_s(&self) -> bool {
        use self::Setting::*;
        self.s == Pressed && self.w != Disabled
    }
    pub fn is_pressed_d(&self) -> bool {
        use self::Setting::*;
        self.d == Pressed && self.a != Disabled
    }

    pub fn direction(&self) -> WasdDirection {
        use self::Setting::*;
        use self::WasdDirection::*;
        if self.is_pressed_w() {
            //W..
            if self.is_pressed_a() {
                WA
            } else if self.is_pressed_d() {
                WD
            } else { W }
        } else if self.is_pressed_s() {
            //S..
            if self.is_pressed_a() {
                SA
            } else if self.is_pressed_d() {
                SD
            } else { S }
        } else {
            //..
            if self.is_pressed_a() {
                A
            } else if self.is_pressed_d() {
                D
            } else { Nothing }
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum WasdDirection {
    Nothing,
    W, A, S, D,
    WA, WD, SA, SD,
}