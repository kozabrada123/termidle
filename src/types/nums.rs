use std::ops;

#[derive(Copy, Clone)]
pub(crate) struct BeegNum {
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

    pub fn add(self, other: BeegNum) -> BeegNum {
        let shift_o = self.shift.saturating_sub(other.shift);
        let shift_s = other.shift.saturating_sub(self.shift);

        return if let Some(new) = (self.value >> shift_s).checked_add(other.value >> shift_o) {
            BeegNum::new(new, self.shift + shift_s + shift_o)
        } else {
            BeegNum::new((self.value >> (1 + shift_s)) + (other.value >> (1 + shift_o)), self.shift + 1 + shift_s + shift_o)
        }
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

#[derive(Copy, Clone)]
pub(crate) struct LittlNum {
    pub value: u8,
    pub shift: u8
}

impl LittlNum {
    pub fn new(value: u8, shift: u8) -> LittlNum {
        LittlNum {
            value,shift
        }
    }

    pub fn add(self, other: LittlNum) -> LittlNum {
        let shift_o = self.shift.saturating_sub(other.shift);
        let shift_s = other.shift.saturating_sub(self.shift);

        return if let Some(new) = (self.value >> shift_s).checked_add(other.value >> shift_o) {
            LittlNum::new(new, self.shift + shift_s + shift_o)
        } else {
            LittlNum::new((self.value >> (1 + shift_s)) + (other.value >> (1 + shift_o)), self.shift + 1 + shift_s + shift_o)
        }
    }
}

impl ops::Add<LittlNum> for LittlNum {
    type Output = LittlNum;

    fn add(self, rhs: LittlNum) -> Self::Output {
        self.add(rhs)
    }
}

impl ops::AddAssign<LittlNum> for LittlNum {
    fn add_assign(&mut self, rhs: LittlNum) {
        let tmp = self.add(rhs);
        self.value = tmp.value;
        self.shift = tmp.shift;
    }
}
