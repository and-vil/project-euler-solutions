use std::fs;

fn main() {
    //Objective: Scan through 1000 consecutive and uniquely ordered digits in data.txt, in 20 rows of 50 digits.
    //Using a 13-digit window, evaluate d_(1) * d_(2) * ... * d_(13) and store the maximum result, then output when complete.
    let pe_digits_file = read_file_string("data.txt"); //data in from file
    let mut pe_digits_str = String::from(""); //init. to empty String, then append input from file
    if let Ok(line) = pe_digits_file {
        pe_digits_str.push_str(line.as_str()); //note: each line in sample file ends in '\r\n', so multiple whitespace chars are present
    } // else if file errors, just ignore it since the rest of the program won't exec. anyhow
    pe_digits_str = remove_whitespace(pe_digits_str.as_str()); //note: String converted to string slice (&str) as function input

    let pe_digits_chars: Vec<char> = pe_digits_str.chars().collect(); //char iterator over digits
    let pe_digits: Vec<u64> = pe_digits_chars //digit iterator, cast to size of final product
        .iter()
        .map(|c| u64::from(c.to_digit(10).unwrap())) //map base-10 char-to-int (.to_digit(10)) safely (.unwrap()), then cast (u64::from)
        .collect();

    let mut max_product: u64 = 0; //final value to return
    let mut current_product: u64; //default init. -should- be 0 for unsigned ints (compiler gripes if init. to 0: "warning: value assigned to `current_product` is never read")

    let digits_windowed = pe_digits.windows(13); //set 13-digit-wide window for calculation
    for window in digits_windowed {
        //thought: branching to omit windows containing zeroes may be cheaper than iterating over all window elements when zeroes guarantee a product of zero
        if window.iter().find(|&&x| x == 0) == None {
            //operable window, no zeroes
            current_product = window.iter().fold(1, |a, &b| a * b); //init. needs to be multiplicative identity for fold to propagate iteratively
            if current_product > max_product {
                max_product = current_product; //update max product
            }
        }
    }
    println!("Max product: {:?}", max_product);
}

//Note: it isn't best practice to read a file in as a single string, but the goal here was more so to focus on conversions and parsing to get used to Rust's data types more
fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?; //read from file at `filepath` into single String, with error propagation (via `?`)
    Ok(data)
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
