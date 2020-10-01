use core::ops::MulAssign;

#[derive(Clone)]
pub struct Powers<T> {
    base: T,
    next_pow: T,
}

impl<T> Powers<T> {
    pub fn new(first: T, base: T) -> Powers<T> {
        Powers {
            base: base,
            next_pow: first,
        }
    }
}

impl<T> Iterator for Powers<T>
where
    T: Copy + MulAssign,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.next_pow;
        self.next_pow *= self.base;
        Some(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn powers2() {
        const BASE: u32 = 2;
        let mut powers = Powers::<u32>::new(1, BASE);
        assert_eq!(powers.next(), Some(1));
        assert_eq!(powers.next(), Some(2));
        assert_eq!(powers.next(), Some(4));
        assert_eq!(powers.next(), Some(8));
        assert_eq!(powers.next(), Some(16));
    }

    #[test]
    fn powers3() {
        const BASE: u32 = 3;
        let mut powers = Powers::<u32>::new(1, BASE);
        assert_eq!(powers.next(), Some(1));
        assert_eq!(powers.next(), Some(3));
        assert_eq!(powers.next(), Some(9));
        assert_eq!(powers.next(), Some(27));
        assert_eq!(powers.next(), Some(81));
    }

    #[test]
    fn powers5() {
        const BASE: u32 = 5;
        let mut powers = Powers::<u32>::new(1, BASE);
        assert_eq!(powers.next(), Some(1));
        assert_eq!(powers.next(), Some(5));
        assert_eq!(powers.next(), Some(25));
        assert_eq!(powers.next(), Some(125));
        assert_eq!(powers.next(), Some(625));
    }
}
