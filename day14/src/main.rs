use grid::*;
fn main() {
    let (p1, p2) = solve(include_str!("../input.txt"));
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn solve(input: &str) -> (i32, i32) {
    let mut grid = Grid::from_vec(
        input.chars().filter(|c| *c != '\n').collect(),
        input.split('\n').next().unwrap().len(),
    );

    let mut p1 = 0;
    for i in 0..grid.cols() {
        let col = grid.iter_col(i).collect::<Vec<_>>();
        let new_col = rock_sort(&col);
        p1 += score_rock(&new_col);
    }

    for _ in 0..1000 {
        for i in 0..grid.cols() {
            let col = grid.iter_col(i).collect::<Vec<_>>();
            let new_col = rock_sort(&col);
            grid.remove_col(i);
            grid.insert_col(i, new_col)
        }

        for i in 0..grid.rows() {
            let row = grid.iter_row(i).collect::<Vec<_>>();
            let new_row = rock_sort(&row);
            grid.remove_row(i);
            grid.insert_row(i, new_row)
        }

        for i in 0..grid.cols() {
            let col = grid.iter_col(i).rev().collect::<Vec<_>>();
            let mut new_col = rock_sort(&col);
            new_col.reverse();
            grid.remove_col(i);
            grid.insert_col(i, new_col)
        }

        for i in 0..grid.rows() {
            let row = grid.iter_row(i).rev().collect::<Vec<_>>();
            let mut new_row = rock_sort(&row);
            new_row.reverse();
            grid.remove_row(i);
            grid.insert_row(i, new_row)
        }
    }

    let mut p2 = 0;
    for i in 0..grid.cols() {
        let col = grid.iter_col(i).collect::<Vec<_>>();
        let scorable = col.iter().map(|&c| *c).collect::<Vec<_>>();
        p2 += score_rock(&scorable);
    }

    (p1, p2)
}

fn rock_sort(col: &Vec<&char>) -> Vec<char> {
    let mut new_col = Vec::new();
    let mut base = 0;
    let mut end = false;

    loop {
        let next_square_rock = match col
            .iter()
            .enumerate()
            .position(|(i, &c)| *c == '#' && i >= base)
        {
            Some(i) => i,
            None => {
                end = true;
                col.len()
            }
        };

        let round_rock = col
            .iter()
            .enumerate()
            .skip(base)
            .filter(|&(i, &c)| *c == 'O' && i < next_square_rock)
            .count();

        new_col.resize(round_rock + base, 'O');
        new_col.resize(next_square_rock, '.');

        if end {
            break;
        }

        new_col.push('#');
        base = next_square_rock + 1;
    }

    new_col
}

fn score_rock(col: &Vec<char>) -> i32 {
    let mut score = 0;
    let cols = col.len();
    for (i, &c) in col.iter().enumerate() {
        if c == 'O' {
            score += (cols - i) as i32;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        let (p1, p2) = solve(input);
        assert_eq!(p1, 136);
        assert_eq!(p2, 64);
    }
}
