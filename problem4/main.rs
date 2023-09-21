use std::collections::HashSet;

const THREE_DIGIT_MIN: u64 = 100;
const THREE_DIGIT_MAX: u64 = 999;

fn main() {
    let mut products: HashSet<u64> = HashSet::new(); //use a HashSet to omit repeated elements
    for i in THREE_DIGIT_MIN..=THREE_DIGIT_MAX {
        for j in THREE_DIGIT_MIN..=THREE_DIGIT_MAX {
            products.insert(i * j);
        }
    }
    println!(
        "Max palindrome product: {:?}",
        products
            .into_iter() //consume `products`
            .filter(|&x| num_is_palindrome(x))
            .max()
            .unwrap()
    );
}

fn num_is_palindrome(n: u64) -> bool {
    let s: Vec<char> = n.to_string().chars().collect::<Vec<char>>();
    return s == s.iter().rev().cloned().collect::<Vec<char>>();
}
