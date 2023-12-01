fn main() {
    let input1 = include_str!("../input1.txt");
    let input2 = include_str!("../input2.txt");
    let official_input = include_str!("../official_input.txt");

    println!("Part 1 Test: {}", part1(input1));
    println!();
    println!();

    println!("Part 1 Official: {}", part1(official_input));
    println!();
    println!();

    println!("Part 2: {}", part2(input2));
    println!();
    println!();

    println!("Part 2 Official: {}", part2(official_input));
    println!();
    println!();
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n");

    for line in lines {
        println!("{}", line);
    }

    return 0;
}

fn part2(input: &str) -> u32 {
    let lines = input.split("\n");

    for line in lines {
        println!("{}", line);
    }

    return 0;
}
