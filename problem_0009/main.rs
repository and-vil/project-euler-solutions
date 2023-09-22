pub fn main() {
    println!("`abc`={:#?}", find_triple().iter().fold(1, |x, y| x*y));
}

fn find_triple() -> Vec<u32> {
    let limit = 1000;
    let mut result: Vec<u32> = vec![];
    for a in 1..limit {
        for b in 1..limit {
            for c in 1..limit {
                if (a) + (b) + (c) == 1000 && (a*a) + (b*b) == (c*c) {result = vec![a, b, c];}
            }
        }
    }
    return result;
}
