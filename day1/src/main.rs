fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let digits = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut total: usize = 0;
    let mut real_total: usize = 0;

    for line in lines {
        let mut first_digit = 0;
        let mut first_digit_index = 0;
        let mut found = false;
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                first_digit = c.to_digit(10).unwrap() as usize;
                first_digit_index = i;
                found = true;
                break;
            }
        }

        let mut real_first_digit = 50;
        for (i, digit) in digits.iter().enumerate() {
            let res: Option<usize> = line.find(digit);
            match res {
                Some(index) => {
                    if index < first_digit_index || !found {
                        real_first_digit = i;
                        first_digit_index = index;
                    }
                },
                None => continue,
            }
        }

        if real_first_digit == 50 {
            real_first_digit = first_digit;
        }
            

        let mut last_digit = 0;
        let mut last_digit_index = 0;
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_digit(10) {
                last_digit = c.to_digit(10).unwrap() as usize;
                last_digit_index = line.len() - i -1;
                break;  
            }
        }

        let mut real_last_digit = 50;
        for (i, digit) in digits.iter().enumerate() {
            let res: Option<usize> = line.rfind(digit);
            match res {
                Some(index) => {
                    if index > last_digit_index {
                        real_last_digit = i;
                        last_digit_index = index;
                    }
                },
                None => continue,
            }
        }

        if real_last_digit == 50 {
            real_last_digit = last_digit;
        }

        let number = first_digit * 10 + last_digit;
        total += number;

        let real_number = real_first_digit * 10 + real_last_digit;
        real_total += real_number;
        
    }

    println!("Day1 Part1:: {}", total);
    println!("Day1 Part2:: {}", real_total);
    
}
