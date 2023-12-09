fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (p1, p2) = solve(input);
    println!("Day 9 part 1: {}", p1);
    println!("Day 9 part 2: {}", p2);
}

fn solve(input: String) -> (i64, i64) {
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    for line in input.lines() {
        let line = line.trim();
        let mut numbers: Vec<Vec<i64>> = Vec::new();
        numbers.push(
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect(),
        );

        loop {
            let mut new_row: Vec<i64> = Vec::new();
            for i in 0..numbers[numbers.len() - 1].len() - 1 {
                new_row.push(numbers[numbers.len() - 1][i + 1] - numbers[numbers.len() - 1][i]);
            }
            numbers.push(new_row);

            let mut all_zero = true;
            for i in 0..numbers[numbers.len() - 1].len() {
                if numbers[numbers.len() - 1][i] != 0 {
                    all_zero = false;
                    break;
                }
            }
            if all_zero {
                break;
            }
        }
        let mut numbers_p2 = numbers.clone();
        for i in 0..numbers.len() - 1 {
            let len = numbers.len();
            let row = numbers[len - 1 - i].clone();
            let next_row = &numbers[len - 2 - i].clone();
            numbers[len - 2 - i].push(next_row[next_row.len() - 1] + row[row.len() - 1]);
        }
        total_p1 += numbers.first().unwrap().last().unwrap();

        for i in 0..numbers_p2.len() - 1 {
            let len = numbers_p2.len();
            let row = numbers_p2[len - 1 - i].clone();
            let next_row = &numbers_p2[len - 2 - i].clone();
            numbers_p2[len - 2 - i].insert(0, next_row[0] - row[0]);
        }
        total_p2 += numbers_p2.first().unwrap().first().unwrap();
    }
    (total_p1, total_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = String::from(
            "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45",
        );
        let (p1, p2) = solve(input);
        assert_eq!(p1, 114);
        assert_eq!(p2, 2);
    }
}
