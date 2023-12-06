fn main() {
    let input1 = include_str!("../input1.txt");
    let input2 = include_str!("../input2.txt");
    let official_input = include_str!("../official_input.txt");
    let expected_output_1 = 0;
    let expected_output_2 = 0;
    let output_1_test = part1(input1);
    let output_2_test = part2(input2);

    if (false == input1.is_empty()) && (true == input2.is_empty()) {
        if expected_output_1 != output_1_test {
            panic!(
                "Failed Part 1. Expected {}, Got {}",
                expected_output_1, output_1_test
            );
        } else {
            println!("Part 1 Official: {}", part1(official_input));
        }
    }

    if false == input2.is_empty() {
        if expected_output_2 != output_2_test {
            panic!(
                "Failed Part 2. Expected {}, Got {}",
                expected_output_2, output_2_test
            );
        } else {
            println!("Part 2 Official: {}", part2(official_input));
        }
    }
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();

    for line in lines {
        println!("{}", line);
    }

    return 0;
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();

    for line in lines {
        println!("{}", line);
    }

    return 0;
}
