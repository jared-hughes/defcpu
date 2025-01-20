use crate::{
    init_mem::{PAGE_SIZE, STACK_TOP},
    Unpredictables,
};

/// TODO-seed: Not really a seed
type Seed = u64;

/// TODO-seed: most terrible prng.
fn silly_prng_byte(seed: Seed, i: u64) -> u8 {
    (((i.wrapping_mul(0x12345678abcdef0)) ^ seed).wrapping_mul(0x1a2b3c4d5e6f708) >> 16) as u8
}

/// TODO-seed: most terrible prng.
fn silly_prng_u32(seed: Seed, i: u64) -> u32 {
    (((i.wrapping_mul(0x1a2b3c4d5e6f708)) ^ seed).wrapping_mul(0x12345678abcdef0) >> 8) as u32
}

/// TODO-correctness: this is nowhere close to what is actually going on.
pub fn gen_unpredictables(seed: Seed) -> Unpredictables {
    let vdso_ptr = STACK_TOP - ((silly_prng_u32(seed, 123) & 0x7FF) as u64) * PAGE_SIZE;
    let stack_top =
        vdso_ptr - 0x1000 * PAGE_SIZE - ((silly_prng_u32(seed, 567) & 0x7FF) as u64) * PAGE_SIZE;
    let argv0_to_platform_offset = (silly_prng_u32(seed, 234) & 0x1FFF) as u64;
    Unpredictables {
        vdso_ptr,
        stack_top,
        platform_offset: argv0_to_platform_offset,
        random_16_bytes: [
            silly_prng_byte(seed, 1001),
            silly_prng_byte(seed, 1002),
            silly_prng_byte(seed, 1013),
            silly_prng_byte(seed, 1034),
            silly_prng_byte(seed, 1085),
            silly_prng_byte(seed, 1116),
            silly_prng_byte(seed, 1237),
            silly_prng_byte(seed, 1478),
            silly_prng_byte(seed, 1301),
            silly_prng_byte(seed, 1202),
            silly_prng_byte(seed, 1113),
            silly_prng_byte(seed, 1534),
            silly_prng_byte(seed, 1485),
            silly_prng_byte(seed, 1716),
            silly_prng_byte(seed, 1537),
            silly_prng_byte(seed, 1978),
        ],
    }
}
