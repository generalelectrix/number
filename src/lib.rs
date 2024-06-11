use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub},
};

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// A float type constrained to the range [0.0, 1.0].
/// The type upholds the range invariant by clamping the value to the range.
#[derive(Display, Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct UnipolarFloat(f64);

impl UnipolarFloat {
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);

    /// Clamp the provided value to the unit range.
    pub fn new(v: f64) -> Self {
        let mut uf = Self(v);
        uf.clamp();
        uf
    }

    /// Return the inner float value.
    pub fn val(&self) -> f64 {
        self.0
    }

    /// Return the negation of this value, mapping 1 to 0 and 0 to 1.
    pub fn invert(&self) -> Self {
        Self(1.0 - self.0)
    }

    fn clamp(&mut self) {
        clamp(&mut self.0, 0.0, 1.0);
    }
}

impl PartialEq<f64> for UnipolarFloat {
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<f64> for UnipolarFloat {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<UnipolarFloat> for f64 {
    fn from(value: UnipolarFloat) -> Self {
        value.0
    }
}

impl Mul for UnipolarFloat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // This cannot go out of range so no need to clamp.
        Self(self.0 * rhs.0)
    }
}

impl Mul<f64> for UnipolarFloat {
    type Output = f64;
    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<UnipolarFloat> for f64 {
    type Output = Self;
    fn mul(self, rhs: UnipolarFloat) -> Self::Output {
        self * rhs.0
    }
}

impl MulAssign for UnipolarFloat {
    fn mul_assign(&mut self, rhs: Self) {
        // This cannot go out of range so no need to clamp.
        self.0 *= rhs.val();
    }
}

impl MulAssign<UnipolarFloat> for f64 {
    fn mul_assign(&mut self, rhs: UnipolarFloat) {
        *self *= rhs.val();
    }
}

impl Sub for UnipolarFloat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl Add for UnipolarFloat {
    type Output = Self;
    /// Add other to self and clamp.
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val() + rhs.val())
    }
}

impl AddAssign for UnipolarFloat {
    /// Add other to self and clamp.
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.val();
    }
}

impl AddAssign<f64> for UnipolarFloat {
    // Add other to self and clamp.
    fn add_assign(&mut self, rhs: f64) {
        *self = Self::new(self.0 + rhs);
    }
}

// A float type constrained to the range [-1.0, 1.0].
/// The type upholds the range invariant by clamping the value to the range.
#[derive(Display, Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct BipolarFloat(f64);

impl BipolarFloat {
    pub const ZERO: Self = Self(0.0);
    pub const ONE: Self = Self(1.0);

    /// Clamp the provided value to the bipolar unit range.
    pub fn new(v: f64) -> Self {
        let mut bf = Self(v);
        bf.clamp();
        bf
    }

    /// Return the inner float value.
    pub fn val(&self) -> f64 {
        self.0
    }

    /// Return the absolute value as a UnipolarFloat.
    pub fn abs(&self) -> UnipolarFloat {
        UnipolarFloat(self.0.abs())
    }

    /// Return the negation of this value.
    pub fn invert(&self) -> Self {
        Self(-1.0 * self.0)
    }

    /// Conditionally return the negation of this value.
    pub fn invert_if(&self, invert: bool) -> Self {
        if invert {
            self.invert()
        } else {
            *self
        }
    }

    fn clamp(&mut self) {
        clamp(&mut self.0, -1.0, 1.0);
    }
}

impl PartialEq<f64> for BipolarFloat {
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<f64> for BipolarFloat {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<BipolarFloat> for f64 {
    fn from(value: BipolarFloat) -> Self {
        value.0
    }
}

impl Mul<UnipolarFloat> for BipolarFloat {
    type Output = Self;
    fn mul(self, rhs: UnipolarFloat) -> Self::Output {
        // This cannot go out of range so no need to clamp.
        Self(self.0 * rhs.0)
    }
}

impl Mul for BipolarFloat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // This cannot go out of range so no need to clamp.
        Self(self.0 * rhs.0)
    }
}

impl Mul<BipolarFloat> for f64 {
    type Output = Self;
    fn mul(self, rhs: BipolarFloat) -> Self::Output {
        self * rhs.0
    }
}

impl Sub for BipolarFloat {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl Add for BipolarFloat {
    type Output = Self;
    /// Add other to self and clamp.
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val() + rhs.val())
    }
}

impl AddAssign for BipolarFloat {
    /// Add other to self and clamp.
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.val();
    }
}

impl AddAssign<f64> for BipolarFloat {
    // Add other to self and clamp.
    fn add_assign(&mut self, rhs: f64) {
        *self = Self::new(self.0 + rhs);
    }
}

fn clamp(v: &mut f64, min: f64, max: f64) {
    *v = f64::min(f64::max(*v, min), max)
}

/// Phase represents a unit angular phase (on the range [0.0, 1.0]).
/// Phase upholds the invariant that the valye contained inside is always in
/// range via wrapping the phase using euclidean modulus.
#[derive(Debug, PartialOrd, Copy, Clone, Serialize, Deserialize, Default)]
pub struct Phase(f64);

impl Phase {
    pub const ZERO: Self = Self(0.0);

    /// Normally this value would always be wrapped back to 0.0, but 1.0 is
    /// an acceptable value for phase and is useful for certain circumstances.
    pub const ONE: Self = Self(1.0);

    pub fn new(v: f64) -> Self {
        let mut p = Self(v);
        p.wrap();
        p
    }

    fn wrap(&mut self) {
        self.0 = self.0.rem_euclid(1.0);
    }

    /// Return the inner phase.
    pub fn val(&self) -> f64 {
        self.0
    }
}

impl From<Phase> for f64 {
    fn from(value: Phase) -> Self {
        value.0
    }
}

impl Add<Phase> for Phase {
    type Output = Phase;
    /// Implement addition as add followed by wrap.
    fn add(self, rhs: Phase) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl Add<f64> for Phase {
    type Output = Phase;
    /// Implement addition as add followed by wrap.
    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl AddAssign<Phase> for Phase {
    fn add_assign(&mut self, rhs: Phase) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for Phase {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Mul<UnipolarFloat> for Phase {
    type Output = Phase;
    fn mul(self, v: UnipolarFloat) -> Self {
        // Can always scale a phase by a unit float as it will never result in
        // an out of range result.
        Self(self.0 * v.val())
    }
}

impl Mul<f64> for Phase {
    type Output = Phase;
    fn mul(self, v: f64) -> Self {
        Self::new(self.0 * v)
    }
}

impl Div<UnipolarFloat> for Phase {
    type Output = Phase;
    /// Divide a phase by a unit float.
    /// The result is wrapped to ensure it is in range.
    fn div(self, v: UnipolarFloat) -> Self {
        Self::new(self.0 / v.val())
    }
}

impl PartialOrd<UnipolarFloat> for Phase {
    fn partial_cmp(&self, other: &UnipolarFloat) -> Option<Ordering> {
        self.0.partial_cmp(&other.val())
    }
}

impl PartialOrd<f64> for Phase {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: Into<f64> + Copy> PartialEq<T> for Phase {
    fn eq(&self, other: &T) -> bool {
        let o: f64 = (*other).into();
        self.0.eq(&o)
    }
}
