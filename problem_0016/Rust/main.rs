use num::BigInt;

fn main() {
    let power = BigInt::from(2u128).pow(1000);
    let sum_of_digits: BigInt = power.to_string().chars().map(|c| c.to_digit(10).unwrap() as u128).map(|d| BigInt::from(d)).sum();
    println!("{}", sum_of_digits);
}
