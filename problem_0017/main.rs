fn main() {
    let numbers: Vec<String> = (1..1001)
        .map(|n| number_as_written(n).replace(" ", "").replace("-", ""))
        .collect();
    let nchars = numbers.iter().fold(0, |acc, x| acc + x.chars().count()); //count chars of each string, not mem. size of string with .len() (*may* be larger)
    println!("Character count: {}", nchars);
}

fn number_as_written(n: usize) -> String {
    //since we're only operating over 1..=1000, we can write a simpler recursive function that's easier to read.
    //writing a more general version would be a bit harder to read (esp. for beginners), and would require more overhead not needed for something this simple.
    //side note: the repeating of .to_owned() eats at me (DRY) but is far quicker to write and still runs fast (suck it, Python - bad code runs FASTER in Rust)
    //read: "TODO: completely refactor, but tactfully"
    let return_str = match n {
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        14 => "fourteen".to_owned(),
        15 => "fifteen".to_owned(),
        16 => "sixteen".to_owned(),
        17 => "seventeen".to_owned(),
        18 => "eighteen".to_owned(),
        19 => "nineteen".to_owned(),
        20 => "twenty".to_owned(),
        30 => "thirty".to_owned(),
        40 => "forty".to_owned(),
        50 => "fifty".to_owned(),
        60 => "sixty".to_owned(),
        70 => "seventy".to_owned(),
        80 => "eighty".to_owned(),
        90 => "ninety".to_owned(),
        1000 => "one thousand".to_owned(),
        _ => {
            let hundreds: usize = n / 100;
            let tens: usize = (n - hundreds * 100) / 10;
            let ones: usize = (n - hundreds * 100) - (tens * 10);
            let mut return_str: String = String::new();
            if hundreds > 0 {
                return_str += &(number_as_written(hundreds) + " hundred");
            }
            if tens < 2 {
                let remainder = n - (hundreds * 100);
                if remainder != 0 {
                    return_str += &(" and ".to_owned() + &number_as_written(n - (hundreds * 100)));
                }
            } else {
                if hundreds > 0 {
                    return_str += " and ";
                }
                return_str += &number_as_written(tens * 10);
                if ones > 0 {
                    return_str += &("-".to_owned() + &number_as_written(ones))
                }
            }
            return return_str;
        }
    };
    return return_str;
}
