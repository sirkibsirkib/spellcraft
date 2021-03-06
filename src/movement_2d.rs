use std::fmt;
use std::f32;
use std::ops;

use std::f32::consts::PI;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector {
    x: f32,
    y: f32,
}
impl Vector {
    pub const NULL: Vector = Vector {x: 0., y: 0.};

    pub fn xy_to_directional(x: f32, y: f32) -> (f32, f32) {
        (
            y.atan2(x) / PI * PI,
            hyp1(x, y),
        )
    }

    pub fn directional_to_xy(direction: f32, speed: f32) -> (f32, f32) {
        (
            direction.cos() * speed,
            direction.sin() * speed,
        )
    }

    #[inline]
    pub fn new_from_xy(x: f32, y: f32) -> Vector {
        Vector { x:x, y:y }
    }

    pub fn rotated(&self, rotation: f32) -> Vector {
        let (dir, spe) = Self::xy_to_directional(self.x, self.y);
        Self::new_from_directional(dir + rotation, spe)
    }

    pub fn new_from_directional(direction: f32, speed: f32) -> Vector {
        let (x, y) = Self::directional_to_xy(direction, speed);
        Self::new_from_xy(x, y)
    }

    pub fn speed(&self) -> f32 {
        hyp1(self.x, self.y)
    }

    pub fn try_set_speed(&mut self, speed: f32, startup_direction: Option<f32>) -> bool {
        let old_speed = self.speed();
        if old_speed == 0. {
            if let Some(dir) = startup_direction {
                *self += Self::new_from_directional(dir, speed);
                return true
            } else {
                return false
            }
        } else {
            *self = *self * (speed / old_speed);
            true
        }
    }

    pub fn slow_by(&mut self, amount: f32) {
        let speed = self.speed();
        if speed == 0. {
            return
        }
        if amount >= speed { //HALT
            self.x = 0.;
            self.y = 0.;
            return
        }
        *self = *self * (speed - amount) / speed;
    }

    #[inline]
    pub fn accelerate_xy(&mut self, x: f32, y:f32) {
        self.x += x;
        self.y += y;
    }

    pub fn accelerate_dirspe(&mut self, direction: f32, speed: f32) {
        let (x, y) = Self::directional_to_xy(direction, speed);
        self.accelerate_xy(x, y)
    }

    #[inline]
    pub fn act_on(&self, pt: &mut Point) {
        pt.apply_vector(self)
    }
}

impl fmt::Debug for Vector {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "<{:?},{:?}>", self.x, self.y)
    }
}

impl ops::Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl ops::Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other
    }
}

impl ops::Div<f32> for Vector {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

macro_rules! sqr {
    ($x:expr) => {{$x*$x}}
}


#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Point(pub f32, pub f32);
impl Point {
    pub const NULL: Point = Point(0., 0.);

    #[inline]
    pub fn dist_to(&self, other: &Self) -> f32 {
        hyp2(self.0, self.1, other.0, other.1)
    }

    pub fn midpoint(pts: &Vec<Point>) -> Option<Point> {
        if pts.len() == 0 { return None }
        let mut mid_pt = Point::NULL;
        for pt in pts.iter() {
            mid_pt.0 += pt.0;
            mid_pt.1 += pt.1;
        }
        mid_pt.0 /= pts.len() as f32;
        mid_pt.1 /= pts.len() as f32;
        Some(mid_pt)
    }

    pub fn direction_to(&self, other: &Point) -> f32 {
        (self.1 - other.1).atan2(self.0 - other.0)
        / PI * PI
    }

    pub fn apply_vector(&mut self, vector: &Vector) {
        self.0 += vector.x;
        self.1 += vector.y;
    }

    pub fn vector_to(&self, other: &Point) -> Vector {
        Vector::new_from_xy(self.0 - other.0, self.1 - other.1)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({:?},{:?})", self.0, self.1)
    }
}


impl ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}
impl ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            0: self.0 - other.0,
            1: self.1 - other.1,
        }
    }
}

impl ops::Mul<f32> for Point {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            0: self.0 * rhs,
            1: self.1 * rhs,
        }
    }
}

impl ops::Div<f32> for Point {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            0: self.0 / rhs,
            1: self.1 / rhs,
        }
    }
}

fn hyp2(ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    (
        sqr![((ax + bx) as f32)] + sqr![((ay + by) as f32)]
    ).sqrt()
}

fn hyp1(x: f32, y: f32) -> f32 {
    (
        sqr![(x as f32)] + sqr![(y as f32)]
    ).sqrt()
}

