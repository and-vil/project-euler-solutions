fn main() {
    let range: Vec<u64> = (2..=20).collect();
    let range_lcm: u64 = range.into_iter().fold(1, |acc, x| simple_lcm(acc, x)); //consume range into folded LCM
    println!("Range LCM: {}", range_lcm);
}

fn simple_lcm(a: u64, b: u64) -> u64 {
    return (a * b) / simple_gcd(a, b);
}

fn simple_gcd(a: u64, b: u64) -> u64 {
    return gcd::binary_u64(a,b); //use binary GCD algorithm
}
