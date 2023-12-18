fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("Day 15, part1: {}", p1);
    println!("Day 15, part2: {}", p2);
}

fn solve(input: &str) -> (i32, i32) {
    let mut total_part1 = 0;

    let steps = input.split(',');
    let mut boxes: Vec<Vec<String>> = Vec::new();

    for _ in 0..255 {
        boxes.push(Vec::new());
    }

    for step in steps {
        total_part1 += hash(step);

        if step.contains('-') {
            let tokens = step.split('-').collect::<Vec<&str>>();
            let label = tokens[0].to_string();
            let boxx = hash(&label);
            for i in 0..boxes[boxx as usize].len() {
                if boxes[boxx as usize][i].contains(&label) {
                    boxes[boxx as usize].remove(i);
                    break;
                }
            }
        } else {
            let tokens = step.split('=').collect::<Vec<&str>>();
            let label = tokens[0].to_string();
            let boxx = hash(&label);
            let lens = tokens[1].parse::<i32>().unwrap();
            let mut found = false;
            let label_len = label.clone() + " " + &lens.to_string();
            for i in 0..boxes[boxx as usize].len() {
                if boxes[boxx as usize][i].contains(&label) {
                    boxes[boxx as usize][i] = label_len.clone();
                    found = true;
                    break;
                }
            }
            if !found {
                boxes[boxx as usize].push(label_len.clone());
            }
        }
    }

    let mut total_part2 = 0;
    for (i, boxx) in boxes.iter().enumerate() {
        for (j, label) in boxx.iter().enumerate() {
            total_part2 += (1 + (i as i32))
                * (1 + (j as i32))
                * label.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        }
    }

    (total_part1, total_part2)
}

fn hash(input: &str) -> i32 {
    let mut current_value = 0;
    for c in input.chars() {
        current_value += c as i32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let (p1, p2) = solve(input);
        assert_eq!(p1, 1320);
        assert_eq!(p2, 145);
    }
}
