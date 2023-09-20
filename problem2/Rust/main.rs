use ndarray::arr2; //import for matrix math

fn main() {
    let one: u64 = 1;
    let fibn_mat = arr2(&[[one, one], [one, 0]]);
    let mut fibn_iter = fibn_mat.dot(&fibn_mat);
    let mut fibseq_values: Vec<u64> = vec![1,1,2]; //statically init. for speed

    while fibn_iter.get((0,0)).unwrap() < &4000000 {
        fibn_iter = fibn_mat.dot(&fibn_iter);
        fibseq_values.push(*fibn_iter.get((0,0)).unwrap());
    }
    let fibseq_even: Vec<u64> = fibseq_values.into_iter().filter(|&x| x%2 == 0).collect(); //filter to only even values
    println!("Even sequence value sum: {:#?}", fibseq_even.into_iter().sum::<u64>()); //consume iterator into sum
}
