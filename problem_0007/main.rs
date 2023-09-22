fn main() {
    //adding timing to this so it isn't just 2 lines of code (though only takes about 200µs to run)
    let clock = std::time::Instant::now(); //mark start of code execution
    let p = primal::StreamingSieve::nth_prime(10001);
    println!("The 10001st prime is: {}", p);
    println!("Elapsed time: {:.2?}", clock.elapsed()); //mark end of code execution
}
