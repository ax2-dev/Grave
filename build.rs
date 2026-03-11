use std::{
    fs::{read_to_string, File},
    io::Write,
};

use rand::{seq::SliceRandom, Rng};

const BASE_OFFSET: u32 = 0xAC564B05;
const BASE_PRIME: u32 = 0x4B9210C9;

fn compute_hash(bytes: &[u8], key: u32) -> u32 {
    let dynamic_offset = BASE_OFFSET ^ key;
    let dynamic_prime = BASE_PRIME.wrapping_add(key);

    let mut hash = dynamic_offset;
    for &b in bytes {
        hash = hash ^ (b as u32);
        hash = (hash << 5) | (hash >> 27);
        hash = hash.wrapping_mul(dynamic_prime);
        hash = hash.wrapping_add(b as u32);
    }
    hash
}

fn main() {
    let mut rng = rand::thread_rng();
    let seed: u32 = rng.r#gen();

    let usernames = read_to_string("usernames.txt").unwrap_or_default();
    let usernames: Vec<&str> = usernames
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut file = File::create("src/consts.rs").unwrap();
    writeln!(file, "pub const COMPILETIME_SEED: u32 = {};", seed).unwrap();
    writeln!(file).unwrap();
    writeln!(file, "pub const SANDBOX_USERNAME_HASHES: &[u32] = &[").unwrap();
    for username in usernames {
        writeln!(
            file,
            "    0x{:08X},",
            compute_hash(username.as_bytes(), seed)
        )
        .unwrap();
    }
    writeln!(file, "];").unwrap();

    let iat_functions: [&str; 60] = [
        "iat_0", "iat_1", "iat_2", "iat_3", "iat_4", "iat_5", "iat_6", "iat_7", "iat_8", "iat_9",
        "iat_10", "iat_11", "iat_12", "iat_13", "iat_14", "iat_15", "iat_16", "iat_17", "iat_18",
        "iat_19", "iat_20", "iat_21", "iat_22", "iat_23", "iat_24", "iat_25", "iat_26", "iat_27",
        "iat_28", "iat_29", "iat_30", "iat_31", "iat_32", "iat_33", "iat_34", "iat_35", "iat_36",
        "iat_37", "iat_38", "iat_39", "iat_40", "iat_41", "iat_42", "iat_43", "iat_44", "iat_45",
        "iat_46", "iat_47", "iat_48", "iat_49", "iat_50", "iat_51", "iat_52", "iat_53", "iat_54",
        "iat_55", "iat_56", "iat_57", "iat_58", "iat_59",
    ];

    let num_to_call = rng.gen_range(5..=10);
    let selected: Vec<_> = iat_functions
        .choose_multiple(&mut rng, num_to_call)
        .cloned()
        .collect();

    writeln!(file).unwrap();
    writeln!(file, "pub fn call_random_iat_functions() {{").unwrap();
    for func in selected {
        writeln!(file, "    crate::iat::{}();", func).unwrap();
    }
    writeln!(file, "}}").unwrap();
}
