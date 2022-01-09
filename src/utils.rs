#[allow(dead_code)]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn hash_version(major: u32, minor: u32, patch: u32, build: u32) -> u32 {
    hash_combine(
        hash_combine(
            hash_combine(
                hash_combine(
                    0,
                    hash_value(build)
                ),
                hash_value(patch)
            ),
            hash_value(minor)
        ),
        hash_value(major)
    ) as u32
}

fn hash_value(mut v: u32) -> u64 {
    v = v.wrapping_shl(15).wrapping_sub(v).wrapping_sub(1);
    v ^= v.wrapping_shr(12);
    v = v.wrapping_add(v << 2);
    v ^= v.wrapping_shr(4);
    v = v.wrapping_mul(2057);
    v ^= v.wrapping_shr(16);

    return v as u64;
}

fn hash_combine(mut seed: u64, mut value: u64) -> u64 {
    const M: u64 = 0xC6A4A7935BD1E995;
    const R: u32 = 47;

    value = value.wrapping_mul(M);
    value ^= value.wrapping_shr(R);
    value = value.wrapping_mul(M);

    seed ^= value;
    seed = seed.wrapping_mul(M);

    seed
}