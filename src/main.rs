use ferrischat_common::types::{UserFlags, GuildFlags};
use std::env;

fn u_parse(flags: i64) {
    let fbits = UserFlags::from_bits_truncate(flags);
    println!("{} - {:?}", flags, fbits)
}

fn g_parse(flags: i64) {
    let fbits = GuildFlags::from_bits_truncate(flags);
    println!("{} - {:?}", flags, fbits)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "u" {
        let flags = args[2].clone();
        let bits = flags.parse::<i64>().unwrap();
        u_parse(bits)
    }
    if args[1] == "g" {
        let flags = args[2].clone();
        let bits = flags.parse::<i64>().unwrap();
        g_parse(bits)
    }
}
