pub struct WasdSet {
    data: u8,
}

impl WasdSet {
    pub fn new() -> WasdSet { WasdSet { data: 0b0000 } }

    pub fn set_w(&mut self) { self.data |= 0b1000; }
    pub fn set_a(&mut self) { self.data |= 0b0100; }
    pub fn set_s(&mut self) { self.data |= 0b0010; }
    pub fn set_d(&mut self) { self.data |= 0b0001; }

    pub fn unset_w(&mut self) { self.data &= 0b0111; }
    pub fn unset_a(&mut self) { self.data &= 0b1011; }
    pub fn unset_s(&mut self) { self.data &= 0b1101; }
    pub fn unset_d(&mut self) { self.data &= 0b1110; }

    pub fn get_w(&self) -> bool { self.data & 0b1000 != 0 }
    pub fn get_a(&self) -> bool { self.data & 0b0100 != 0 }
    pub fn get_s(&self) -> bool { self.data & 0b0010 != 0 }
    pub fn get_d(&self) -> bool { self.data & 0b0001 != 0 }
}