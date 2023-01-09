pub(crate) trait Functions {
    fn ch(x: Self, y: Self, z: Self) -> Self;
    fn maj(x: Self, y: Self, z: Self) -> Self;

    fn sum0(x: Self) -> Self;
    fn sum1(x: Self) -> Self;

    fn sigma0(x: Self) -> Self;
    fn sigma1(x: Self) -> Self;
}

impl Functions for u32 {
    #[inline(always)]
    fn ch(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ ((!x) & z)
    }

    #[inline(always)]
    fn maj(x: u32, y: u32, z: u32) -> u32 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    #[inline(always)]
    fn sum0(x: u32) -> u32 {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }

    #[inline(always)]
    fn sum1(x: u32) -> u32 {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }

    #[inline(always)]
    fn sigma0(x: u32) -> u32 {
        x.rotate_right(7) ^ x.rotate_right(18) ^ x.wrapping_shr(3)
    }

    #[inline(always)]
    fn sigma1(x: u32) -> u32 {
        x.rotate_right(17) ^ x.rotate_right(19) ^ x.wrapping_shr(10)
    }
}

impl Functions for u64 {
    #[inline(always)]
    fn ch(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ ((!x) & z)
    }

    #[inline(always)]
    fn maj(x: u64, y: u64, z: u64) -> u64 {
        (x & y) ^ (x & z) ^ (y & z)
    }

    #[inline(always)]
    fn sum0(x: u64) -> u64 {
        x.rotate_right(28) ^ x.rotate_right(34) ^ x.rotate_right(39)
    }

    #[inline(always)]
    fn sum1(x: u64) -> u64 {
        x.rotate_right(14) ^ x.rotate_right(18) ^ x.rotate_right(41)
    }

    #[inline(always)]
    fn sigma0(x: u64) -> u64 {
        x.rotate_right(1) ^ x.rotate_right(8) ^ x.wrapping_shr(7)
    }

    #[inline(always)]
    fn sigma1(x: u64) -> u64 {
        x.rotate_right(19) ^ x.rotate_right(61) ^ x.wrapping_shr(6)
    }
}
