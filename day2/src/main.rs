use regex::Regex;

fn main() {
    let input = include_str!("input.txt").lines();

    let re = Regex::new(r"^(\d+)\-(\d+) (.): (.+)$").unwrap();

    let mut count = 0;
    for line in input {
        println!("{}", line);
        assert!(re.is_match(line));
        let caps = re.captures(line).unwrap();
        println!(
            "are from {} to {} characters '{}' in \"{}\"?",
            &caps[1], &caps[2], &caps[3], &caps[4]
        );

        let actual_number = caps[4].matches(&caps[3]).count();
        println!("{}", actual_number);
        if actual_number >= caps[1].parse::<usize>().unwrap()
            && actual_number <= caps[2].parse::<usize>().unwrap()
        {
            count += 1;
        }
    }
    println!("{} pwds match their rule.", count);
}
