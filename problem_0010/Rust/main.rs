use primal::Primes;

const PRIME_N_LIMIT: usize = 148933; //n of nth closest prime under 2M: 1999993

fn main() {
    //again, add clock in (1) out of curiosity, and (2) because this takes 2 lines of code
    let clock = std::time::Instant::now(); //mark start of code execution
    let prime_sum: usize = Primes::all().take(PRIME_N_LIMIT).sum(); //sum all primes under 2M
    println!("{}", prime_sum);
    println!("Elapsed time: {:.2?}", clock.elapsed()); //mark end of code execution
}
