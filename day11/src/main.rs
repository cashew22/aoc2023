fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Unable to read input.txt");
    println!("Day11 Part1: {}", solve(input.clone(), 2));
    println!("Day11 Part2: {}", solve(input, 1000000));
}

fn solve(input: String, expand_ratio: i64) -> i64 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut expanded_grid = expand(&grid, 2);
    let mut galaxies = get_galaxies(&expanded_grid);
    let distances_base = get_distances(&galaxies);

    if expand_ratio == 2 {
        return distances_base.iter().map(|(_, _, distance)| distance).sum();
    }

    expanded_grid = expand(&grid, 10);
    galaxies = get_galaxies(&expanded_grid);
    let distances = get_distances(&galaxies);

    let mut total_distance: i64 = 0;
    for i in 0..distances.len() {
        let base = distances_base[i].2 as i64;
        let distance = distances[i].2 as i64;
        let ratio = distance - base;
        let factor = ratio / 8;

        total_distance += base + ratio + factor * (expand_ratio - 10);
    }

    total_distance
}

fn get_galaxies(grid: &Vec<Vec<char>>) -> Vec<(i64, i64, i64)> {
    let mut galaxies: Vec<(i64, i64, i64)> = Vec::new();
    let mut galaxy_count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                galaxy_count += 1;
                galaxies.push((galaxy_count, i as i64, j as i64));
            }
        }
    }
    galaxies
}

fn get_distances(galaxies: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64, i64)> {
    let mut distances: Vec<(i64, i64, i64)> = Vec::new();
    for galaxy_1 in galaxies {
        for galaxy_2 in galaxies {
            if galaxy_1 == galaxy_2 {
                continue;
            }

            let (id1, x1, y1) = galaxy_1;
            let (id2, x2, y2) = galaxy_2;
            if id2 < id1 {
                continue;
            }
            let distance = (x1 - x2).abs() + (y1 - y2).abs();

            distances.push((id1.clone(), id2.clone(), distance));
        }
    }

    distances
}

fn expand(grid: &Vec<Vec<char>>, expand_ratio: i64) -> Vec<Vec<char>> {
    let mut expanded_grid = Vec::new();
    for line in grid {
        expanded_grid.push(line.clone());
        let dots = line.iter().filter(|c| **c == '.').count();
        if dots == line.len() {
            for _ in 0..expand_ratio - 1 {
                expanded_grid.push(line.clone());
            }
        }
    }

    let mut indexes_to_expand = Vec::new();

    for i in 0..expanded_grid[0].len() {
        let mut dots = 0;
        for j in 0..expanded_grid.len() {
            if expanded_grid[j][i] == '.' {
                dots += 1;
            }
        }

        for _ in 0..expand_ratio - 1 {
            if dots == expanded_grid.len() {
                indexes_to_expand.push(i);
            }
        }
    }

    let mut added = 0;
    for index in indexes_to_expand {
        for line in &mut expanded_grid {
            line.insert(index + added, '.');
        }
        added += 1;
    }

    expanded_grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = String::from(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(solve(input.clone(), 2), 374);
        assert_eq!(solve(input.clone(), 10), 1030);
        assert_eq!(solve(input.clone(), 100), 8410);
    }
}
