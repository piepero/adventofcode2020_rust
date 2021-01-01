/* adventofcode2020

day 1

with a lot of help from
https://fasterthanli.me/series/advent-of-code-2020/part-1

my initial, brute force, approach worked (manually inserted the input as Vec<u32> in the source code),
but was very un-rusty.
*/
use itertools::Itertools;

const EXPECTED_PRODUCT: u32 = 2020;

fn main() {
    let input = include_str!("input.txt")
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap);

    let (a, b) = input
        .clone()
        .tuple_combinations()
        .find(|(a, b)| a + b == EXPECTED_PRODUCT)
        .unwrap();

    println!("{} * {} = {}", a, b, a * b);

    let (a, b, c) = input
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == EXPECTED_PRODUCT)
        .unwrap();

    println!("{} * {} * {} = {}", a, b, c, a * b * c);
}
