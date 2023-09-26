fn main() {
    let mut max_chain_len = 0;
    let mut max_chain_val: u64 = 0;
    let mut collatz_run_len: u64;
    for i in 15..1_000_000 {
        collatz_run_len = collatz_chain_len(i);
        if collatz_run_len > max_chain_len {
            max_chain_len = collatz_run_len;
            max_chain_val = i;
        }
    }
    //Runs in about 140ms, may benefit from multithreading to parallelize (as loop iterations in `main()` are functionally independent)...
    println!(
        "Max chain length starting from {:#?}: {:#?}",
        max_chain_val, max_chain_len
    );
}

fn collatz_chain_len(mut n: u64) -> u64 {
    let mut counter = 1;
    loop {
        n = match n % 2 {
            0 => n / 2,
            _ => 3 * n + 1,
        };
        counter += 1;
        if n == 1 {
            break;
        }
    }
    return counter;
}
