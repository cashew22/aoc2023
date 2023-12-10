fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (p1, p2) = solve(input);
    println!("Day 10, part 1: {}", p1);
    println!("Day 10, part 2: {}", p2);
}

fn solve(input: String) -> (i64, i64) {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut pos = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                pos = (i as i32, j as i32);
            }
        }
    }
    let mut positions = vec![pos];

    let mut dir = "down";
    let mut steps = 0;
    loop {
        pos = match dir {
            "up" => (pos.0 - 1, pos.1),
            "right" => (pos.0, pos.1 + 1),
            "down" => (pos.0 + 1, pos.1),
            "left" => (pos.0, pos.1 - 1),
            _ => panic!("Invalid direction"),
        };
        steps += 1;

        let pipe = map[pos.0 as usize][pos.1 as usize];
        positions.push(pos);

        dir = match pipe {
            '|' => dir,
            '-' => dir,
            'L' => match dir {
                "down" => "right",
                "left" => "up",
                _ => panic!("Invalid direction:"),
            },

            'J' => match dir {
                "down" => "left",
                "right" => "up",
                _ => panic!("Invalid direction"),
            },
            '7' => match dir {
                "up" => "left",
                "right" => "down",
                _ => panic!("Invalid direction"),
            },
            'F' => match dir {
                "up" => "right",
                "left" => "down",
                _ => panic!("Invalid direction"),
            },
            'S' => break,
            _ => panic!("Invalid pipe"),
        };
    }

    let poly = geometry_rs::Polygon::new(
        positions
            .iter()
            .map(|p| geometry_rs::Point {
                x: p.1 as f64,
                y: p.0 as f64,
            })
            .collect::<Vec<geometry_rs::Point>>(),
        vec![],
    );

    let mut points_in_poly = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let point = geometry_rs::Point {
                x: j as f64,
                y: i as f64,
            };
            if poly.contains_point(point) {
                if !positions.contains(&(i as i32, j as i32)) {
                    points_in_poly += 1;
                }
            }
        }
    }

    (steps / 2, points_in_poly)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = ".....
        .S-7.
        .|.|.
        .L-J.
        .....";
        let (p1, _) = solve(input.to_string());
        assert_eq!(p1, 4);
    }

    #[test]
    fn test_part1_2() {
        let input = "..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";
        let (p1, _) = solve(input.to_string());
        assert_eq!(p1, 8);
    }

    #[test]
    fn test_part2_1() {
        let input = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";
        let (_, p2) = solve(input.to_string());
        assert_eq!(p2, 4);
    }

    #[test]
    fn test_part2_2() {
        let input = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";
        let (_, p2) = solve(input.to_string());
        assert_eq!(p2, 8);
    }

    #[test]
    fn test_part2_3() {
        let input = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";
        let (_, p2) = solve(input.to_string());
        assert_eq!(p2, 10);
    }
}
