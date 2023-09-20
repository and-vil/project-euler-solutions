fn main() {
    let mut nums: Vec<u32> = (3..1000).collect(); //effectively a type cast
    nums = nums.into_iter().filter(|&x| x%3 == 0 || x%5 == 0).collect(); //filter to values to sum
    println!("{:#?}", nums.into_iter().sum::<u32>()); //sum filtered values
}
