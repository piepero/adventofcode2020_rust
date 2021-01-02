fn trees_hit(right: usize, down: usize) -> usize {
    // a very hacky implementation, not robust at all
    let input = include_str!("input.txt").lines();

    let mut trees_hit: usize = 0;

    let mut position: usize = 0;
    let mut line_no: usize = 0;

    for line in input {
        line_no += 1;
        // println!("{}[{}] @ {}", line, line.len(), position % line.len());

        if down == 2 && line_no % 2 == 0 {
            continue;
        }

        if line.chars().nth(position % line.len()).unwrap() == '#' {
            trees_hit += 1;
        }
        position += right;
    }

    println!("{} trees hit on slope {}/{}", trees_hit, right, down);

    trees_hit
}

fn main() {
    println!(
        "Solution: {}",
        trees_hit(1, 1) * trees_hit(3, 1) * trees_hit(5, 1) * trees_hit(7, 1) * trees_hit(1, 2)
    );
}
