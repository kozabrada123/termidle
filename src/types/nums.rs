use std::{cmp, ops};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct BeegNum {
    pub value: u128,
    pub shift: u128
}
/*
    the idea behind this number being that when u128 doesnt hold enough bits we can drop precision
 */
impl BeegNum {
    pub fn new(value: u128, shift: u128) -> BeegNum {
        BeegNum {
            value,shift
        }
    }

    pub fn truncate(self) -> BeegNum {
        let to_truncate = self.value.leading_zeros() as u128;

        return if self.value == 0 {
            BeegNum::new(0, 0)
        } else if self.shift >= to_truncate {
            BeegNum::new(self.value << to_truncate, self.shift - to_truncate)
        } else {
            BeegNum::new(self.value << self.shift, 0)
        }
    }

    pub fn add(self, other: BeegNum) -> BeegNum {
        let shift_o = self.shift.saturating_sub(other.shift);
        let shift_s = other.shift.saturating_sub(self.shift);

        if shift_o > 127 {return self}
        if shift_s > 127 {return other}

        return if let Some(new) = (self.value >> shift_s).checked_add(other.value >> shift_o) {
            BeegNum::new(new, self.shift + shift_s + shift_o)
        } else {
            BeegNum::new((self.value >> (1 + shift_s)) + (other.value >> (1 + shift_o)), self.shift + 1 + shift_s + shift_o)
        }
    }

    pub fn sub(self, other: BeegNum) -> BeegNum {
        let shift_o = self.shift.saturating_sub(other.shift);

        if shift_o > 127 {return self}

        BeegNum::new(self.value.saturating_sub(other.value >> shift_o), self.shift)
    }

    pub fn eq(self, other: BeegNum) -> bool {
        let tr_self = self.truncate();
        let tr_other = other.truncate();
        tr_self.value == tr_other.value && tr_self.shift == tr_other.shift
    }

    pub fn lt(self, other: BeegNum) -> bool {
        let tr_self = self.truncate();
        let tr_other = other.truncate();
        (tr_self.shift == tr_other.shift && tr_self.value < tr_other.value) || (tr_self.shift < tr_other.shift)
    }

    pub fn gt(self, other: BeegNum) -> bool {
        let tr_self = self.truncate();
        let tr_other = other.truncate();
        (tr_self.shift == tr_other.shift && tr_self.value > tr_other.value) || (tr_self.shift > tr_other.shift)
    }
}

impl ops::Add<BeegNum> for BeegNum {
    type Output = BeegNum;

    fn add(self, rhs: BeegNum) -> Self::Output {
        self.add(rhs)
    }
}

impl ops::AddAssign<BeegNum> for BeegNum {
    fn add_assign(&mut self, rhs: BeegNum) {
        let tmp = self.add(rhs);
        self.value = tmp.value;
        self.shift = tmp.shift;
    }
}

impl ops::Sub<BeegNum> for BeegNum {
    type Output = BeegNum;

    fn sub(self, rhs: BeegNum) -> Self::Output {
        self.sub(rhs)
    }
}

impl ops::SubAssign<BeegNum> for BeegNum {
    fn sub_assign(&mut self, rhs: BeegNum) {
        let tmp = self.sub(rhs);
        self.value = tmp.value;
        self.shift = tmp.shift;
    }
}

impl PartialEq<BeegNum> for BeegNum {
    fn eq(&self, other: &BeegNum) -> bool {
        self.eq(other)
    }

    fn ne(&self, other: &BeegNum) -> bool {
        !self.eq(other)
    }
}

impl Eq for BeegNum {}

impl PartialOrd<BeegNum> for BeegNum {
    fn partial_cmp(&self, other: &BeegNum) -> Option<Ordering> {
        return if self.lt(&other) {
            Some(Ordering::Less)
        } else if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.gt(&other) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }

    fn lt(&self, other: &BeegNum) -> bool {
        self.lt(&other)
    }

    fn le(&self, other: &BeegNum) -> bool {
        self.lt(&other) || self.eq(other)
    }

    fn gt(&self, other: &BeegNum) -> bool {
        self.gt(&other)
    }

    fn ge(&self, other: &BeegNum) -> bool {
        self.gt(&other) || self.eq(other)
    }
}
