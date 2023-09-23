fn main() {
    let test1 = String::from("We need more spaces!");
    assert_eq!(trim_spaces(&test1), "We need more spaces!");

    let test1 = String::from("   We need more spaces!");
    assert_eq!(trim_spaces(&test1), "We need more spaces!");

    let test1 = String::from("We need more spaces!    ");
    assert_eq!(trim_spaces(&test1), "We need more spaces!");

    let test1 = String::from("        We need more spaces!     ");
    assert_eq!(trim_spaces(&test1), "We need more spaces!");

    let test1 = String::from("      ");
    assert_eq!(trim_spaces(&test1), "");
}

fn trim_spaces(input: &String) -> &str {
    let mut start = 0;
    let mut end = 0;
    let length = input.len();
    // Eliminate starting spaces
    for (i, c) in input.chars().enumerate() {
        if c != ' ' {
            start = i;
            break;
        }
    }

    // Eliminate ending Spaces
    for i in 0..length {
        if input.as_bytes()[length - i - 1] != b' ' {
            end = length - i;
            break;
        }
    }

    let trimmed_string = &input[start..end];
    trimmed_string
}
