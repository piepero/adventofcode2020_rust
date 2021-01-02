#[macro_use]
extern crate lazy_static;

use regex::Regex;

fn conforms_to_rule_1(line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)\-(\d+) (.): (.+)$").unwrap();
    }
    let caps = RE.captures(line).unwrap();

    let character_count = caps[4].matches(&caps[3]).count();
    let min_repeats = caps[1].parse::<usize>().unwrap();
    let max_repeats = caps[2].parse::<usize>().unwrap();

    character_count >= min_repeats && character_count <= max_repeats
}

fn conforms_to_rule_2(line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)\-(\d+) (.): (.+)$").unwrap();
    }
    let caps = RE.captures(line).unwrap();

    let pos1 = caps[1].parse::<usize>().unwrap();
    let pos2 = caps[2].parse::<usize>().unwrap();
    let character: char = caps[3].chars().next().unwrap();

    let char_at_pos1 = caps[4].chars().nth(pos1 - 1).unwrap();
    let char_at_pos2 = caps[4].chars().nth(pos2 - 1).unwrap();

    ((char_at_pos1 == character) || (char_at_pos2 == character))
        && !((char_at_pos1 == character) && (char_at_pos2 == character))
}

fn main() {
    let input = include_str!("input.txt").lines();

    println!(
        "{} passwords conform to rule 1",
        input.clone().filter(|x| conforms_to_rule_1(x)).count()
    );

    println!(
        "{} passwords conform to rule 2",
        input.filter(|x| conforms_to_rule_2(x)).count()
    );
}
