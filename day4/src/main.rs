
struct Card {
    _id: i32,
    winning_numbers : Vec<i32>,
    numbers_you_have : Vec<i32>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut cards: Vec<Card> = Vec::new();
    let mut cards_count: Vec<i32> = Vec::new();

    let mut total_part1 = 0;
    for line in input.lines() {
        let mut card = Card {
            _id: line.split(":").nth(0).unwrap().split_whitespace().nth(1).unwrap().parse::<i32>().unwrap(),
            winning_numbers: Vec::new(),
            numbers_you_have: Vec::new(),
        };

        card.winning_numbers = line.split(":").nth(1).unwrap().split("|").nth(0).unwrap().trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        card.numbers_you_have = line.split(":").nth(1).unwrap().split("|").nth(1).unwrap().trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        cards.push(card);
        cards_count.push(1);
    }


    for (i, card) in cards.iter().enumerate() {
        let mut points = 0;

        for (j, _number) in card.numbers_you_have.iter().filter(|x| card.winning_numbers.contains(x)).enumerate() {
            points = if j == 0 { 1 } else { points * 2 };
            cards_count[i+j+1] += cards_count[i];
        }
        println!("Card {} has {} points", card._id, points);
        total_part1 += points;
    }

    println!("Part 1: {}", total_part1);
    println!("Part 2: {}", cards_count.iter().sum::<i32>());
}
