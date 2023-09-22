fn main() {
    let range: Vec<u64> = (1..101).collect();
    let sum_of_sqares: u64 = range.iter().map(|&x|x * x).sum();
    let mut square_of_sum: u64 = range.iter().sum();
    square_of_sum *= square_of_sum;
    println!("Sum of squares: {}", sum_of_sqares);
    println!("Square of sum: {}", square_of_sum);
    println!("Difference: {}", square_of_sum - sum_of_sqares);
}
