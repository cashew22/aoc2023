use fancy_regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (part1, part2) = solve(input);
    println!("Day7 part1: {}", part1);
    println!("Day7 part2: {}", part2);
}

fn solve(input: String) -> (i64, i64) {
    let mut hands: Vec<String> = Vec::new();
    let mut sorted_hands: Vec<String> = Vec::new();
    let mut bets: HashMap<String, i64> = HashMap::new();

    for (hand, bet) in input
        .lines()
        .map(|line| line.split_whitespace())
        .map(|mut line| (line.next().unwrap(), line.next().unwrap()))
    {
        hands.push(hand.to_string());
        let mut chars = hand.chars().collect::<Vec<_>>();
        chars.sort_by(|a, b| b.cmp(a));
        sorted_hands.push(chars.into_iter().collect::<String>());
        bets.insert(hand.to_string(), bet.parse::<i64>().unwrap());
    }

    let mut joker_hands = hands.clone();

    let mut actual_rank: i64 = input.lines().count().try_into().unwrap();
    let regexes = vec![
        Regex::new(r"(.)\1{4}").unwrap(),
        Regex::new(r"(.)\1{3}").unwrap(),
        Regex::new(r"((.)\2\2(.)\3|(.)\4(.)\5\5)").unwrap(),
        Regex::new(r"(.)\1{2}").unwrap(),
        Regex::new(r".*(.)\1.*(.)\2.*").unwrap(),
        Regex::new(r"(.)\1{1}").unwrap(),
        Regex::new(r".").unwrap(),
    ];

    let mut hands_part1 = hands.clone();
    let mut score_p1 = 0;
    for regex in &regexes {
        score_p1 += score_hands(
            &regex,
            &mut hands_part1,
            &mut sorted_hands,
            &bets,
            &mut actual_rank,
            compare_hands,
        );
    }

    let mut best_hands = Vec::new();
    for hand in &mut joker_hands {
        let mut regex_strength = 6;
        let mut best_hand = hand.clone();
        best_hand = optimize_joker_hand(hand, best_hand, &regexes, &mut regex_strength);
        best_hands.push(best_hand);
    }

    let mut score_p2 = 0;
    let mut actual_rank: i64 = input.lines().count().try_into().unwrap();
    for regex in &regexes {
        score_p2 += score_hands(
            &regex,
            &mut hands,
            &mut best_hands,
            &bets,
            &mut actual_rank,
            compare_hands_part2,
        );
    }

    (score_p1, score_p2)
}

const CARDS: [&str; 12] = ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"];

fn optimize_joker_hand(
    hand: &mut String,
    best_hand: String,
    regexes: &Vec<Regex>,
    regex_strength: &mut usize,
) -> String {
    if !hand.contains('J') {
        for i in 0..regexes.len() {
            if regexes[i].is_match(hand).unwrap() {
                if i <= *regex_strength {
                    *regex_strength = i;
                    let mut new_hand = hand.clone();
                    let mut chars = new_hand.chars().collect::<Vec<_>>();
                    chars.sort_by(|a, b| b.cmp(a));
                    new_hand = chars.into_iter().collect::<String>();
                    return new_hand;
                }
            }
        }
        return best_hand;
    }

    let mut best_hand = best_hand;
    for card in CARDS.iter() {
        let mut new_hand = hand.clone();
        new_hand = new_hand.replacen('J', card, 1);
        let mut chars = new_hand.chars().collect::<Vec<_>>();
        chars.sort_by(|a, b| b.cmp(a));
        new_hand = chars.into_iter().collect::<String>();
        best_hand = optimize_joker_hand(&mut new_hand, best_hand, regexes, regex_strength);
    }
    best_hand
}

fn score_hands(
    regex: &Regex,
    hands: &mut Vec<String>,
    sorted_hands: &mut Vec<String>,
    bets: &HashMap<String, i64>,
    actual_rank: &mut i64,
    compare: fn(&String, &String) -> std::cmp::Ordering,
) -> i64 {
    let mut hands_to_score = Vec::new();
    let mut to_remove_from_sorted = Vec::new();
    for (i, sorted_hand) in sorted_hands.iter().enumerate() {
        let results = regex.captures(sorted_hand);
        match results {
            Ok(Some(_)) => {
                hands_to_score.push(hands[i].to_string());
                to_remove_from_sorted.push(sorted_hand.to_string());
            }
            _ => {}
        }
    }

    hands_to_score.sort_by(compare);
    let mut score = 0;
    for hand in hands_to_score {
        score += bets.get(&hand).unwrap() * *actual_rank;
        *actual_rank -= 1;
        let index = hands.iter().position(|x| x == &hand).unwrap();
        hands.remove(index);
        sorted_hands.remove(index);
    }

    score
}

fn compare_hands(a: &String, b: &String) -> std::cmp::Ordering {
    let a_chars = a.chars().collect::<Vec<_>>();
    let b_chars = b.chars().collect::<Vec<_>>();
    for i in 0..a_chars.len() {
        let a_value = match a_chars[i] {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => a_chars[i].to_digit(10).unwrap(),
        };
        let b_value = match b_chars[i] {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => b_chars[i].to_digit(10).unwrap(),
        };
        if a_value < b_value {
            return std::cmp::Ordering::Greater;
        } else if a_value > b_value {
            return std::cmp::Ordering::Less;
        } else {
            continue;
        }
    }
    std::cmp::Ordering::Equal
}

fn compare_hands_part2(a: &String, b: &String) -> std::cmp::Ordering {
    let a_chars = a.chars().collect::<Vec<_>>();
    let b_chars = b.chars().collect::<Vec<_>>();
    for i in 0..a_chars.len() {
        let a_value = match a_chars[i] {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 2,
            'T' => 11,
            _ => a_chars[i].to_digit(10).unwrap() + 1,
        };
        let b_value = match b_chars[i] {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 2,
            'T' => 11,
            _ => b_chars[i].to_digit(10).unwrap() + 1,
        };
        if a_value < b_value {
            return std::cmp::Ordering::Greater;
        } else if a_value > b_value {
            return std::cmp::Ordering::Less;
        } else {
            continue;
        }
    }
    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQJAQ 483";
        let (part1, part2) = solve(input.to_string());

        assert_eq!(part1, 6440);
        assert_eq!(part2, 5905);
    }
}
