pub fn sign_extend_32s64(x: u32) -> u64 {
    ((x as i32 as i64) << 32 >> 32) as u64
}

#[cfg(test)]
mod test_bit_hacks {
    use super::*;

    #[test]
    fn test_sign_extend_32s64() {
        assert_eq!(sign_extend_32s64(0x0000_1234), 0x0000_0000_0000_1234);
        assert_eq!(sign_extend_32s64(0x0000_7FFF), 0x0000_0000_0000_7FFF);
        assert_eq!(sign_extend_32s64(0x1234_5678), 0x0000_0000_1234_5678);
        assert_eq!(sign_extend_32s64(0x7FFF_FFFF), 0x0000_0000_7FFF_FFFF);
        assert_eq!(sign_extend_32s64(0x8000_0000), 0xFFFF_FFFF_8000_0000);
        assert_eq!(sign_extend_32s64(0xF123_4567), 0xFFFF_FFFF_F123_4567);
    }
}
