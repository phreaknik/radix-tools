use core::ops::{Div, Mul, Sub};

pub struct RadixDecomposer<T> {
    r: T,
    q: T,
}

impl<T> RadixDecomposer<T> {
    pub fn new(radix: T, num: T) -> RadixDecomposer<T> {
        RadixDecomposer { r: radix, q: num }
    }
}

impl<T> Iterator for RadixDecomposer<T>
where
    T: Copy + Div<Output = T> + Mul<Output = T> + Sub<Output = T>,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.q;
        self.q = self.q / self.r;
        Some(curr - self.r * self.q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::powers::*;

    #[test]
    fn radix2_decomp() {
        const RADIX: u32 = 2;
        let digits = vec![1, 0, 0, 1];
        let value = Powers::<u32>::new(1, RADIX)
            .zip(digits.iter())
            .map(|(b, d)| b * d)
            .sum();
        let mut decomp = RadixDecomposer::<u32>::new(RADIX, value);
        for digit in digits {
            assert_eq!(Some(digit), decomp.next());
        }
    }

    #[test]
    fn radix3_decomp() {
        const RADIX: u32 = 3;
        let digits = vec![1, RADIX - 1, 0, 1];
        let value = Powers::<u32>::new(1, RADIX)
            .zip(digits.iter())
            .map(|(b, d)| b * d)
            .sum();
        let mut decomp = RadixDecomposer::<u32>::new(RADIX, value);
        for digit in digits {
            assert_eq!(Some(digit), decomp.next());
        }
    }

    #[test]
    fn radix5_decomp() {
        const RADIX: u32 = 5;
        let digits = vec![1, RADIX - 1, 0, 1];
        let value = Powers::<u32>::new(1, RADIX)
            .zip(digits.iter())
            .map(|(b, d)| b * d)
            .sum();
        let mut decomp = RadixDecomposer::<u32>::new(RADIX, value);
        for digit in digits {
            assert_eq!(Some(digit), decomp.next());
        }
    }
}
