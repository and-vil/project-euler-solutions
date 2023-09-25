use {
    std::fs::File,
    std::io::{self, BufRead, BufReader},
    std::path::Path,
    num_bigint::BigUint
};

fn main() {
    //Read data into String Vec (with error handling!)
    let pe_number_strings: Vec<String> = match lines_from_file("data.txt") {
        Ok(lines) => lines,
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    let numbers: Vec<BigUint> = pe_number_strings.into_iter().map(|x| x.parse::<BigUint>().unwrap()).collect();
    let sum: BigUint = numbers.into_iter().sum::<BigUint>();
    let result: String = sum.to_string();
    println!("{:#?}", &result[..10]); //slice first 10 chars of string
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
