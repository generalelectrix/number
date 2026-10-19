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
    ///
    /// Reject NaN; set it to 0 instead.
    #[inline]
    pub const fn new(v: f64) -> Self {
        if v.is_nan() {
            debug_assert!(false, "NaN passed to UnipolarFloat");
            return Self::ZERO;
        }
        Self(v.clamp(0.0, 1.0))
    }

    /// Return the inner float value.
    #[inline]
    pub const fn val(self) -> f64 {
        self.0
    }

    /// Return the negation of this value, mapping 1 to 0 and 0 to 1.
    #[inline]
    pub const fn invert(self) -> Self {
        Self(1.0 - self.0)
    }

    /// Return this value as a Phase.
    #[inline]
    pub const fn as_phase(self) -> Phase {
        // Phase and Unipolar have the same domain, no need to check.
        Phase(self.0)
    }

    /// Rescale this unipolar number into a bipolar number.
    /// 0 -> -1.0, 0.5 -> 0, 1.0 -> 1.0
    #[inline]
    pub const fn rescale_as_bipolar(self) -> BipolarFloat {
        BipolarFloat((self.0 * 2.0) - 1.0)
    }
}

impl PartialEq<f64> for UnipolarFloat {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<f64> for UnipolarFloat {
    #[inline]
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<UnipolarFloat> for f64 {
    #[inline]
    fn from(value: UnipolarFloat) -> Self {
        value.0
    }
}

impl Mul for UnipolarFloat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        // This cannot go out of range so no need to clamp.
        Self(self.0 * rhs.0)
    }
}

impl Mul<f64> for UnipolarFloat {
    type Output = f64;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        self.0 * rhs
    }
}

impl Mul<UnipolarFloat> for f64 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: UnipolarFloat) -> Self::Output {
        self * rhs.0
    }
}

impl MulAssign for UnipolarFloat {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        // This cannot go out of range so no need to clamp.
        self.0 *= rhs.val();
    }
}

impl MulAssign<UnipolarFloat> for f64 {
    #[inline]
    fn mul_assign(&mut self, rhs: UnipolarFloat) {
        *self *= rhs.val();
    }
}

impl Sub for UnipolarFloat {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl Add for UnipolarFloat {
    type Output = Self;
    /// Add other to self and clamp.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val() + rhs.val())
    }
}

impl AddAssign for UnipolarFloat {
    /// Add other to self and clamp.
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.val();
    }
}

impl AddAssign<f64> for UnipolarFloat {
    // Add other to self and clamp.
    #[inline]
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
    ///
    /// Reject NaN; set it to 0 instead.
    #[inline]
    pub const fn new(v: f64) -> Self {
        if v.is_nan() {
            debug_assert!(false, "NaN passed to BipolarFloat");
            return Self::ZERO;
        }
        Self(v.clamp(-1.0, 1.0))
    }

    /// Return the inner float value.
    #[inline]
    pub const fn val(self) -> f64 {
        self.0
    }

    /// Return the absolute value as a UnipolarFloat.
    #[inline]
    pub const fn abs(self) -> UnipolarFloat {
        UnipolarFloat(self.0.abs())
    }

    /// Return the negation of this value.
    #[inline]
    pub const fn invert(self) -> Self {
        Self(-self.0)
    }

    /// Conditionally return the negation of this value.
    #[inline]
    pub const fn invert_if(self, invert: bool) -> Self {
        if invert { self.invert() } else { self }
    }

    /// Rescale this bipolar number into a unipolar number.
    /// -1 -> 0, 0 -> 0.5, 1.0 -> 1.0
    #[inline]
    pub const fn rescale_as_unipolar(self) -> UnipolarFloat {
        UnipolarFloat((self.0 + 1.0) / 2.0)
    }
}

impl PartialEq<f64> for BipolarFloat {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<f64> for BipolarFloat {
    #[inline]
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<BipolarFloat> for f64 {
    #[inline]
    fn from(value: BipolarFloat) -> Self {
        value.0
    }
}

impl Mul<UnipolarFloat> for BipolarFloat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: UnipolarFloat) -> Self::Output {
        // This cannot go out of range so no need to clamp.
        Self(self.0 * rhs.0)
    }
}

impl Mul for BipolarFloat {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        // This cannot go out of range so no need to clamp.
        Self(self.0 * rhs.0)
    }
}

impl Mul<BipolarFloat> for f64 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: BipolarFloat) -> Self::Output {
        self * rhs.0
    }
}

impl Sub for BipolarFloat {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0)
    }
}

impl Add for BipolarFloat {
    type Output = Self;
    /// Add other to self and clamp.
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val() + rhs.val())
    }
}

impl AddAssign for BipolarFloat {
    /// Add other to self and clamp.
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.val();
    }
}

impl AddAssign<f64> for BipolarFloat {
    // Add other to self and clamp.
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        *self = Self::new(self.0 + rhs);
    }
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

    /// Wrap the provided value to [0, 1) to represent a phase.
    ///
    /// Reject a value that isn't finite (NaN or an infinity); set it to 0
    /// instead.
    #[inline]
    pub fn new(v: f64) -> Self {
        let wrapped = Self::wrap(v);
        // NaN wraps to NaN, and so does an infinity, whose distance above its
        // floor is inf - inf: one check on the result catches both. A branch
        // to a cold path, rather than a select, keeps it off the wrap's chain.
        if wrapped.is_nan() {
            return Self::rejected();
        }
        Self(wrapped)
    }

    /// The value's distance above the whole number at or below it: the
    /// Euclidean remainder by one, with every zero positive.
    ///
    /// A negative value too close to zero for its distance below one to be
    /// told apart from one comes out as exactly 1.0.
    #[inline]
    fn wrap(v: f64) -> f64 {
        // Rounding down rather than toward zero is what lifts a negative
        // value into range, so no correction follows the subtraction.
        v - v.floor()
    }

    /// Zero, standing in for a value that has no phase.
    ///
    /// Takes nothing, so a caller has nothing to keep alive for it.
    #[cold]
    #[inline(never)]
    fn rejected() -> Self {
        debug_assert!(false, "a value that isn't finite passed to Phase");
        Self::ZERO
    }

    /// Return this phase as a UnipolarFloat.
    #[inline]
    pub fn as_unipolar(&self) -> UnipolarFloat {
        // Phase and Unipolar have the same domain, no need to check.
        UnipolarFloat(self.0)
    }

    /// Return the inner phase.
    #[inline]
    pub fn val(&self) -> f64 {
        self.0
    }
}

impl From<Phase> for f64 {
    #[inline]
    fn from(value: Phase) -> Self {
        value.0
    }
}

impl Add<Phase> for Phase {
    type Output = Phase;
    /// Implement addition as add followed by wrap.
    #[inline]
    fn add(self, rhs: Phase) -> Self::Output {
        Self::new(self.0 + rhs.0)
    }
}

impl Add<f64> for Phase {
    type Output = Phase;
    /// Implement addition as add followed by wrap.
    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.0 + rhs)
    }
}

impl AddAssign<Phase> for Phase {
    #[inline]
    fn add_assign(&mut self, rhs: Phase) {
        *self = *self + rhs;
    }
}

impl AddAssign<f64> for Phase {
    #[inline]
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
    }
}

impl Mul<UnipolarFloat> for Phase {
    type Output = Phase;
    #[inline]
    fn mul(self, v: UnipolarFloat) -> Self {
        // Can always scale a phase by a unit float as it will never result in
        // an out of range result.
        Self(self.0 * v.val())
    }
}

impl Mul<f64> for Phase {
    type Output = Phase;
    #[inline]
    fn mul(self, v: f64) -> Self {
        Self::new(self.0 * v)
    }
}

impl Div<UnipolarFloat> for Phase {
    type Output = Phase;
    /// Divide a phase by a unit float.
    /// The result is wrapped to ensure it is in range.
    #[inline]
    fn div(self, v: UnipolarFloat) -> Self {
        // A full duty cycle divides by exactly one, which changes nothing but
        // costs a divide on the caller's chain. Wrapped all the same, so
        // `Phase::ONE` still comes out as zero.
        if v.0 == 1.0 {
            return Self::new(self.0);
        }
        Self::new(self.0 / v.val())
    }
}

impl PartialOrd<UnipolarFloat> for Phase {
    #[inline]
    fn partial_cmp(&self, other: &UnipolarFloat) -> Option<Ordering> {
        self.0.partial_cmp(&other.val())
    }
}

impl PartialOrd<f64> for Phase {
    #[inline]
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<T: Into<f64> + Copy> PartialEq<T> for Phase {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        let o: f64 = (*other).into();
        self.0.eq(&o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Finite values across every range a phase is made from, and either
    /// side of whole numbers and eighths, the same on every run.
    fn probes() -> Vec<f64> {
        // xorshift64*.
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        let mut next = move || {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            state.wrapping_mul(0x2545_F491_4F6C_DD1D)
        };
        let mut probes = Vec::new();
        for _ in 0..100_000 {
            let unit = (next() >> 11) as f64 / (1u64 << 53) as f64;
            probes.extend([unit, unit * 64.0, unit * 1e6, -unit * 1e6, -unit * 1e-12]);
            // Any finite value at all; a phase is never made from anything else.
            let any = f64::from_bits(next());
            if any.is_finite() {
                probes.push(any);
            }
        }
        // Whole numbers and eighths, and the values either side of them.
        for k in -64..=64 {
            let (mut below, mut above) = (f64::from(k) / 8.0, f64::from(k) / 8.0);
            for _ in 0..4 {
                probes.extend([below, above]);
                below = below.next_down();
                above = above.next_up();
            }
        }
        probes
    }

    /// A phase is the distance above the whole number at or below the value
    /// it was made from: the Euclidean remainder by one, with every zero
    /// positive.
    #[test]
    fn a_phase_is_the_euclidean_remainder_by_one_with_positive_zeros() {
        for v in probes() {
            let wrapped = Phase::new(v).val();
            let remainder = v.rem_euclid(1.0);
            if remainder == 0.0 {
                assert!(
                    wrapped == 0.0 && wrapped.is_sign_positive(),
                    "{v:e} wrapped to {wrapped:e} rather than +0"
                );
            } else {
                assert_eq!(
                    wrapped.to_bits(),
                    remainder.to_bits(),
                    "{v:e} wrapped to {wrapped:e} rather than {remainder:e}"
                );
            }
        }
        assert!(Phase::new(-3.0).val().is_sign_positive());
        // Too little below zero for its distance below one to be told apart
        // from one.
        assert_eq!(Phase::new(-1e-20).val(), 1.0);
    }

    /// Dividing a phase wraps the quotient, whatever the divisor.
    #[test]
    fn dividing_a_phase_wraps_the_quotient() {
        let divisors = [0.5, 0.7, 1.0 - f64::EPSILON, 1.0].map(UnipolarFloat::new);
        for v in probes() {
            let p = Phase::new(v);
            for d in divisors {
                let quotient = (p / d).val();
                let wrapped = Phase::new(p.val() / d.val()).val();
                assert_eq!(
                    quotient.to_bits(),
                    wrapped.to_bits(),
                    "{p:?} / {d:?} gave {quotient:e} rather than {wrapped:e}"
                );
            }
        }
        assert_eq!((Phase::ONE / UnipolarFloat::ONE).val(), 0.0);
    }

    /// A value that isn't finite has no phase. It's refused loudly where
    /// debug assertions are on, and stands in as zero where they're off.
    #[test]
    fn a_value_that_is_not_finite_is_refused() {
        type Make = fn() -> Phase;
        let makers: [(&str, Make); 5] = [
            ("NaN", || Phase::new(f64::NAN)),
            ("infinity", || Phase::new(f64::INFINITY)),
            ("negative infinity", || Phase::new(f64::NEG_INFINITY)),
            ("a phase over zero", || {
                Phase::new(0.5) / UnipolarFloat::ZERO
            }),
            ("a phase over -0", || {
                Phase::new(0.5) / UnipolarFloat::new(-0.0)
            }),
        ];
        for (what, make) in makers {
            let made = std::panic::catch_unwind(make);
            if cfg!(debug_assertions) {
                assert!(made.is_err(), "{what} was let through as {made:?}");
            } else {
                let made = made.map(|p| p.val().to_bits());
                assert_eq!(
                    made.ok(),
                    Some(0.0f64.to_bits()),
                    "{what} did not stand in as zero"
                );
            }
        }
    }
}
