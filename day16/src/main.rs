use std::collections::HashMap;

use grid::*;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (p1, p2) = solve(&input);
    println!("Day16, Part 1: {}", p1);
    println!("Day16, Part 2: {}", p2);
}

#[derive(Hash, Eq, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Beam {
    col: i32,
    row: i32,
    direction: Direction,
}

fn solve(input: &str) -> (i32, i32) {
    let grid = Grid::from_vec(
        input.chars().filter(|c| *c != '\n').collect(),
        input.find('\n').unwrap(),
    );

    let mut p1 = 0;
    let mut p2 = Vec::new();
    for row in 0..grid.rows() {
        let mut tracker = grid.clone();
        let mut map: HashMap<Beam, bool> = HashMap::new();
        launch_beam(
            &grid,
            &mut map,
            &mut tracker,
            0,
            row as i32,
            Direction::Right,
        );
        let mut res = tracker.iter().filter(|c| **c == '#').count() as i32;
        p2.push(res);
        if p1 == 0 {
            p1 = res;
        }

        tracker = grid.clone();
        map.clear();
        launch_beam(
            &grid,
            &mut map,
            &mut tracker,
            (grid.cols() - 1) as i32,
            row as i32,
            Direction::Left,
        );
        res = tracker.iter().filter(|c| **c == '#').count() as i32;

        p2.push(res);
    }

    for col in 0..grid.cols() {
        let mut tracker = grid.clone();
        let mut map: HashMap<Beam, bool> = HashMap::new();
        launch_beam(
            &grid,
            &mut map,
            &mut tracker,
            col as i32,
            0,
            Direction::Down,
        );
        let mut res = tracker.iter().filter(|c| **c == '#').count() as i32;
        p2.push(res);

        tracker = grid.clone();
        map.clear();
        launch_beam(
            &grid,
            &mut map,
            &mut tracker,
            col as i32,
            (grid.rows() - 1) as i32,
            Direction::Up,
        );
        res = tracker.iter().filter(|c| **c == '#').count() as i32;

        p2.push(res);
    }

    (p1, *p2.iter().max().unwrap())
}

fn launch_beam(
    grid: &Grid<char>,
    map: &mut HashMap<Beam, bool>,
    tracker: &mut Grid<char>,
    start_col: i32,
    start_row: i32,
    start_direction: Direction,
) {
    let mut beam = Beam {
        col: start_col,
        row: start_row,
        direction: start_direction.clone(),
    };

    loop {
        if map.contains_key(&beam) {
            break;
        }

        if beam.col < 0
            || beam.row < 0
            || beam.col >= grid.cols() as i32
            || beam.row >= grid.rows() as i32
        {
            break;
        }

        let position = (beam.row as usize, beam.col as usize);
        tracker[position] = '#';
        map.insert(beam.clone(), true);

        match grid.get(position.0, position.1) {
            Some('.') => match beam.direction {
                Direction::Up => {
                    beam.row -= 1;
                }
                Direction::Down => {
                    beam.row += 1;
                }
                Direction::Left => {
                    beam.col -= 1;
                }
                Direction::Right => {
                    beam.col += 1;
                }
            },
            Some('\\') => match beam.direction {
                Direction::Up => {
                    beam.direction = Direction::Left;
                    beam.col -= 1;
                }
                Direction::Down => {
                    beam.direction = Direction::Right;
                    beam.col += 1;
                }
                Direction::Left => {
                    beam.direction = Direction::Up;
                    beam.row -= 1;
                }
                Direction::Right => {
                    beam.direction = Direction::Down;
                    beam.row += 1;
                }
            },
            Some('/') => match beam.direction {
                Direction::Up => {
                    beam.direction = Direction::Right;
                    beam.col += 1;
                }
                Direction::Down => {
                    beam.direction = Direction::Left;
                    beam.col -= 1;
                }
                Direction::Left => {
                    beam.direction = Direction::Down;
                    beam.row += 1;
                }
                Direction::Right => {
                    beam.direction = Direction::Up;
                    beam.row -= 1;
                }
            },
            Some('|') => match beam.direction {
                Direction::Up => {
                    beam.row -= 1;
                }
                Direction::Down => {
                    beam.row += 1;
                }
                Direction::Left | Direction::Right => {
                    launch_beam(grid, map, tracker, beam.col, beam.row + 1, Direction::Down);
                    launch_beam(grid, map, tracker, beam.col, beam.row - 1, Direction::Up);
                    break;
                }
            },
            Some('-') => match beam.direction {
                Direction::Up | Direction::Down => {
                    launch_beam(grid, map, tracker, beam.col - 1, beam.row, Direction::Left);
                    launch_beam(grid, map, tracker, beam.col + 1, beam.row, Direction::Right);
                    break;
                }
                Direction::Left => {
                    beam.col -= 1;
                }
                Direction::Right => {
                    beam.col += 1;
                }
            },
            None => {
                break;
            }
            _ => {
                println!("Beam hit something else");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        let (p1, p2) = solve(input);
        assert_eq!(p1, 46);
        assert_eq!(p2, 51);
    }
}
