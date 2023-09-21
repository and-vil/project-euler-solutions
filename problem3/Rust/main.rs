use prime_factorization::Factorization;

fn main() {
    let num: u64 = 600851475143;
    println!(
        "Largest prime factor of {:#?}: {:#?}",
        num,
        Factorization::run(num).factors.iter().max().unwrap()
    );
}
