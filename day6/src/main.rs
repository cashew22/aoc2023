use roots;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let times = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().collect::<Vec<_>>();
    let distances = input.lines().nth(1).unwrap().split(":").nth(1).unwrap().split_whitespace().collect::<Vec<_>>();

    let mut prod = 1.0;
    for i in 0..times.len() {
        let time = times[i].parse::<f64>().unwrap();
        let distance = distances[i].parse::<f64>().unwrap();
        match roots::find_roots_quadratic(-1.0, time, -distance) {
            roots::Roots::Two(r) => {
                prod *= (r[1].ceil() - r[0].floor()) -1.0;
            }
            _ => println!("No roots found"),
        }
    }

    println!("Day6 Part1: {}", prod);

    let time = input.lines().nth(0).unwrap().split(":").nth(1).unwrap().split_whitespace().collect::<String>().parse::<f64>().unwrap();
    let distance = input.lines().nth(1).unwrap().split(":").nth(1).unwrap().split_whitespace().collect::<String>().parse::<f64>().unwrap();
    match roots::find_roots_quadratic(-1.0, time, -distance) {
        roots::Roots::Two(r) => {
            println!("Day6 Part2: {}", (r[1].ceil() - r[0].floor()) -1.0);
        }
        _ => println!("No roots found"),
    }
    
}
