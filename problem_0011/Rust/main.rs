use std::{env, fs};

enum ProductDirection {
    Vertical,
    Horizontal,
    Diagonal,
    DiagonalRev
}

fn main() {
    //Objective: read in `data.txt`, which contains 20 rows of 2-digit fixed-width numbers in [01, 99];
    //Searching for the greatest product of four adjacent numbers in the same direction
    //(i.e. up, down, left, right, or diagonally) in the 20x20 grid.

    //set backtrace for exec. on this file if error is present (used in testing, not needed for final exec.)
    env::set_var("RUST_BACKTRACE", "1");

    let pe_digits_file: Result<String, Box<dyn std::error::Error>> = read_file_string("data.txt"); //data in from file
    let mut pe_digits_str: String = String::from(""); //init. to empty String, then append input from file
    if let Ok(line) = pe_digits_file {
        pe_digits_str.push_str(line.as_str()); //note: each line in sample file ends in '\r\n', so multiple whitespace chars are present
    } else {
        println!("File read error! Check your formatting.");
    }
    let file_rows: Vec<String> = pe_digits_str.split("\r\n").map(str::to_string).collect();

    let mut strings: Vec<&str>; //init per-line string-slice storage (needed for cast to int)
    let mut line_nums: Vec<u32>; //init per-line number storage to push to `all_nums`
    let mut all_nums: [[u32; 20]; 20] = [[0; 20]; 20];

    //fill array with converted Strings -> ints
    let mut counter: usize = 0;
    let mut inner_counter: usize = 0;
    while counter < file_rows.len() {
        strings = file_rows[counter].split(' ').collect();
        line_nums = strings.iter().map(|&x| x.parse::<u32>().unwrap()).collect();
        while inner_counter <= 19 {
            all_nums[counter][inner_counter] = line_nums[inner_counter];
            inner_counter += 1;
        }
        inner_counter = 0;
        counter += 1;
    }

    //Next:
    // (1) Create vertical, horizontal, and diagonal convolution windows across the grid
    // (2) For each convolution window, compute a product, then store max.

    let mut maxprods: Vec<u32> = vec![];
    let prod_types = [ProductDirection::Vertical, ProductDirection::Horizontal, ProductDirection::Diagonal, ProductDirection::DiagonalRev];
    for prod_type in prod_types {
        maxprods.push(max_product(&all_nums, prod_type));
    }
    println!("Max product: {:#?}", maxprods.iter().max().unwrap());
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?; //read from file at `filepath` into single String, with error propagation (via `?`)
    Ok(data)
}

fn max_product(grid: &[[u32; 20]; 20], flag: ProductDirection) -> u32 {
    //valid `flag` values: 'vertical', 'horizontal', 'diagonal', and 'diagonal_rev'
    //`diagonal_rev` captures inverted diagonal product directionality, as diagonal products aren't spatially invertible like vertical/horizonal are

    let mut max_product: u32 = 0;
    let limit: usize = grid.len() - 1;

    let row_start: usize = match flag {
        ProductDirection::Vertical => 3,
        ProductDirection::Diagonal => 3,
        ProductDirection::DiagonalRev => 3,
        _ => 0,
    };

    let col_start: usize = match flag {
        ProductDirection::Horizontal => 3,
        ProductDirection::Diagonal => 3,
        _ => 0,
    };

    let col_end: usize = match flag {
        ProductDirection::DiagonalRev => 3,
        _ => 0,
    };

    //`offsets`: row 0 = row offsets, row 2 = col offsets
    //cols of `offsets` are effectively element position indices
    let offsets_neg: [[usize; 4]; 2] = match flag {
        ProductDirection::Vertical | ProductDirection::DiagonalRev => [[0, 1, 2, 3], [0, 0, 0, 0]],
        ProductDirection::Horizontal => [[0, 0, 0, 0], [0, 1, 2, 3]],
        _ => [[0, 1, 2, 3], [0, 1, 2, 3]], //diagonal = default
    };

    let offsets_pos: [usize; 4] = match flag {
        ProductDirection::DiagonalRev => [0, 1, 2, 3], //only increment in case of reverse diagonal; only inc. on cols.
        _ => [0, 0, 0, 0],
    };

    let mut current_vproduct: u32;
    for row in row_start..=limit {
        for col in col_start..=(limit - col_end) {
            current_vproduct = grid[row - offsets_neg[0][0]][col - offsets_neg[1][0] + offsets_pos[0]]
                * grid[row - offsets_neg[0][1]][col - offsets_neg[1][1] + offsets_pos[1]]
                * grid[row - offsets_neg[0][2]][col - offsets_neg[1][2] + offsets_pos[2]]
                * grid[row - offsets_neg[0][3]][col - offsets_neg[1][3] + offsets_pos[3]];
            max_product = if current_vproduct > max_product { current_vproduct } else { max_product };
        }
    }

    return max_product;
}
