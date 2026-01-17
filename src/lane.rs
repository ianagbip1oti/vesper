#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Lane(pub __m256i);

const NOT_A_FILE: u64 = !0x0101010101010101u64;
const NOT_H_FILE: u64 = !0x8080808080808080u64;

impl Lane {
    pub const EMPTY: Self = unsafe { Self(_mm256_setzero_si256()) };

    #[inline]
    pub fn new(a: u64, b: u64, c: u64, d: u64) -> Self {
        unsafe {
            Self(_mm256_set_epi64x(d as i64, c as i64, b as i64, a as i64))
        }
    }

    #[inline]
    pub fn from_single(val: u64) -> Self {
        unsafe {
            Self(_mm256_set1_epi64x(val as i64))
        }
    }

    #[inline]
    pub fn extract(&self, index: usize) -> u64 {
        unsafe {
            let mut arr = [0u64; 4];
            _mm256_storeu_si256(arr.as_mut_ptr() as *mut __m256i, self.0);
            arr[index]
        }
    }

    #[inline]
    pub fn eq(&self, other: Lane) -> Lane {
        unsafe { Self(_mm256_cmpeq_epi64(self.0, other.0)) }
    }

    #[inline]
    pub fn is_zero_mask(&self) -> Lane {
        self.eq(Self::EMPTY)
    }

    #[inline]
    pub fn is_not_zero_mask(&self) -> Lane {
        !self.is_zero_mask()
    }

    #[inline]
    pub fn shift_north(&self) -> Self {
        unsafe { Self(_mm256_slli_epi64(self.0, 8)) }
    }

    #[inline]
    pub fn shift_south(&self) -> Self {
        unsafe { Self(_mm256_srli_epi64(self.0, 8)) }
    }

    #[inline]
    pub fn shift_east(&self) -> Self {
        unsafe {
            let shifted = _mm256_slli_epi64(self.0, 1);
            let mask = _mm256_set1_epi64x(NOT_A_FILE as i64);
            Self(_mm256_and_si256(shifted, mask))
        }
    }

    #[inline]
    pub fn shift_west(&self) -> Self {
        unsafe {
            let shifted = _mm256_srli_epi64(self.0, 1);
            let mask = _mm256_set1_epi64x(NOT_H_FILE as i64);
            Self(_mm256_and_si256(shifted, mask))
        }
    }

    #[inline]
    pub fn shift_north_east(&self) -> Self {
        unsafe {
            let shifted = _mm256_slli_epi64(self.0, 9);
            let mask = _mm256_set1_epi64x(NOT_A_FILE as i64);
            Self(_mm256_and_si256(shifted, mask))
        }
    }

    #[inline]
    pub fn shift_north_west(&self) -> Self {
        unsafe {
            let shifted = _mm256_slli_epi64(self.0, 7);
            let mask = _mm256_set1_epi64x(NOT_H_FILE as i64);
            Self(_mm256_and_si256(shifted, mask))
        }
    }

    #[inline]
    pub fn shift_south_east(&self) -> Self {
        unsafe {
            let shifted = _mm256_srli_epi64(self.0, 7);
            let mask = _mm256_set1_epi64x(NOT_A_FILE as i64);
            Self(_mm256_and_si256(shifted, mask))
        }
    }

    #[inline]
    pub fn shift_south_west(&self) -> Self {
        unsafe {
            let shifted = _mm256_srli_epi64(self.0, 9);
            let mask = _mm256_set1_epi64x(NOT_H_FILE as i64);
            Self(_mm256_and_si256(shifted, mask))
        }
    }

    pub fn fill_north(&self, empty: Lane) -> Lane {
        let mut g = *self;
        g |= empty & g.shift_north();
        let mut e = empty & empty.shift_north();
        g |= e & g.shift_north().shift_north();
        e &= e.shift_north().shift_north();
        g |= e & g.shift_north().shift_north().shift_north().shift_north();
        g
    }

    pub fn fill_south(&self, empty: Lane) -> Lane {
        let mut g = *self;
        g |= empty & g.shift_south();
        let mut e = empty & empty.shift_south();
        g |= e & g.shift_south().shift_south();
        e &= e.shift_south().shift_south();
        g |= e & g.shift_south().shift_south().shift_south().shift_south();
        g
    }

    pub fn fill_east(&self, empty: Lane) -> Lane {
        let mut g = *self;
        g |= empty & g.shift_east();
        let mut e = empty & empty.shift_east();
        g |= e & g.shift_east().shift_east();
        e &= e.shift_east().shift_east();
        g |= e & g.shift_east().shift_east().shift_east().shift_east();
        g
    }

    pub fn fill_west(&self, empty: Lane) -> Lane {
        let mut g = *self;
        g |= empty & g.shift_west();
        let mut e = empty & empty.shift_west();
        g |= e & g.shift_west().shift_west();
        e &= e.shift_west().shift_west();
        g |= e & g.shift_west().shift_west().shift_west().shift_west();
        g
    }

    pub fn fill_north_east(&self, empty: Lane) -> Lane {
        let mut g = *self;
        g |= empty & g.shift_north_east();
        let mut e = empty & empty.shift_north_east();
        g |= e & g.shift_north_east().shift_north_east();
        e &= e.shift_north_east().shift_north_east();
        g |= e & g.shift_north_east().shift_north_east().shift_north_east().shift_north_east();
        g
    }

    pub fn fill_north_west(&self, empty: Lane) -> Lane {
        let mut g = *self;
        g |= empty & g.shift_north_west();
        let mut e = empty & empty.shift_north_west();
        g |= e & g.shift_north_west().shift_north_west();
        e &= e.shift_north_west().shift_north_west();
        g |= e & g.shift_north_west().shift_north_west().shift_north_west().shift_north_west();
        g
    }

    pub fn fill_south_east(&self, empty: Lane) -> Lane {
        let mut g = *self;
        g |= empty & g.shift_south_east();
        let mut e = empty & empty.shift_south_east();
        g |= e & g.shift_south_east().shift_south_east();
        e &= e.shift_south_east().shift_south_east();
        g |= e & g.shift_south_east().shift_south_east().shift_south_east().shift_south_east();
        g
    }

    pub fn fill_south_west(&self, empty: Lane) -> Lane {
        let mut g = *self;
        g |= empty & g.shift_south_west();
        let mut e = empty & empty.shift_south_west();
        g |= e & g.shift_south_west().shift_south_west();
        e &= e.shift_south_west().shift_south_west();
        g |= e & g.shift_south_west().shift_south_west().shift_south_west().shift_south_west();
        g
    }

    pub fn knight_attacks(&self) -> Self {
        let n = self.shift_north();
        let s = self.shift_south();
        let e = self.shift_east();
        let w = self.shift_west();

        (n.shift_north_east() | n.shift_north_west()) |
        (s.shift_south_east() | s.shift_south_west()) |
        (e.shift_north_east() | e.shift_south_east()) |
        (w.shift_north_west() | w.shift_south_west())
    }

    pub fn king_attacks(&self) -> Self {
        let n = self.shift_north();
        let s = self.shift_south();
        let e = self.shift_east();
        let w = self.shift_west();
        n | s | e | w | n.shift_east() | n.shift_west() | s.shift_east() | s.shift_west()
    }
}

impl BitAnd for Lane {
    type Output = Self;
    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        unsafe { Self(_mm256_and_si256(self.0, rhs.0)) }
    }
}

impl BitOr for Lane {
    type Output = Self;
    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        unsafe { Self(_mm256_or_si256(self.0, rhs.0)) }
    }
}

impl BitXor for Lane {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        unsafe { Self(_mm256_xor_si256(self.0, rhs.0)) }
    }
}

impl Not for Lane {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        unsafe {
            let ones = _mm256_set1_epi64x(-1);
            Self(_mm256_xor_si256(self.0, ones))
        }
    }
}

impl BitAndAssign for Lane {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOrAssign for Lane {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXorAssign for Lane {
    #[inline]
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl std::fmt::Debug for Lane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Lane({:016x}, {:016x}, {:016x}, {:016x})",
            self.extract(0), self.extract(1), self.extract(2), self.extract(3))
    }
}
