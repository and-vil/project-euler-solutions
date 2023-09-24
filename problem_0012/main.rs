fn main() {
    let mut nn: u64 = 1;
    while tau(triangular_number(nn)) < 500 {
        nn += 1;
    }
    println!("Triangular number: {}", triangular_number(nn)); //check out https://oeis.org/A084260 after finding the result...
}

fn triangular_number(n: u64) -> u64 {
    return n * (n + 1) / 2;
}

//--latter two functions are from Rosetta Code for tau fn.--
// Returns the highest power of `i` that is a factor of `n`, and `n` divided by that power of `i`
fn factor_exponent(n: u64, i: u64) -> (u64, u64) {
    if n % i == 0 {
        let (a, b) = factor_exponent(n / i, i);
        (a + 1, b)
    } else {
        (0, n)
    }
}

//AKA sigma_0(n), returns count of divisors of `n`
fn tau(n: u64) -> u64 {
    for i in 2..(n + 1) {
        if n % i == 0 {
            let (count, next) = factor_exponent(n, i);
            return (count + 1) * tau(next);
        }
    }
    return 1;
}

//Note: just iterating over `n%i==0` and incrementing a counter if `i` is a divisor is very inefficient
//so this is a much more efficient method of expediting that process with something more sieve-like.
