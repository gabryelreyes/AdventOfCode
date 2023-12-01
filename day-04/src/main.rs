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
    let mut total_points: u32 = 0;

    for line in lines {
        let line_parts: Vec<&str> = line.split(":").collect();
        // let card_number: u32 = line_parts[0].split_whitespace().collect::<Vec<&str>>()[1]
        //     .parse()
        //     .unwrap();
        let numbers = line_parts[1].split("|").collect::<Vec<&str>>();
        let winning_numbers: Vec<u32> = numbers[0]
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        let my_numbers: Vec<u32> = numbers[1]
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();

        // println!("{}", card_number);
        // println!("{:?}", winning_numbers);
        // println!("{:?}", my_numbers);
        // println!("");

        let mut matches: u32 = 0;

        for possible in &my_numbers {
            for winner in &winning_numbers {
                if possible == winner {
                    // println!("{}", &possible);
                    matches += 1;
                }
            }
        }
        // println!();

        let base: u32 = 2;
        let points = base.pow(matches);

        total_points += points / 2;
    }

    return total_points;
}

fn part2(input: &str) -> u32 {
    let lines = input.split("\n");
    let mut total_points: u32 = 0;

    let lines_vector: Vec<&str> = lines.clone().collect();

    let mut number_of_cards: Vec<u32> = vec![1; lines_vector.len()];

    for line in lines {
        let line_parts: Vec<&str> = line.split(":").collect();
        let card_number: usize = line_parts[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let numbers = line_parts[1].split("|").collect::<Vec<&str>>();
        let winning_numbers: Vec<u32> = numbers[0]
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        let my_numbers: Vec<u32> = numbers[1]
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();

        // println!("{}", card_number);
        // println!("{:?}", winning_numbers);
        // println!("{:?}", my_numbers);
        // println!("");

        let mut matches: usize = 0;

        for possible in &my_numbers {
            for winner in &winning_numbers {
                if possible == winner {
                    // println!("{}", &possible);
                    matches += 1;
                }
            }
        }
        // println!();

        let exemplars: u32 = number_of_cards[card_number - 1];

        for card in card_number..(&card_number + &matches) {
            if card <= (number_of_cards.len() - 1) {
                number_of_cards[card] += exemplars;
            }
        }

        // dbg!(&card_number);
        // dbg!(&exemplars);
        // dbg!(&number_of_cards);

        // let base: u32 = 2;
        // let points = base.pow(matches);

        // total_points += points / 2;
    }

    for number in number_of_cards {
        total_points += number;
    }

    return total_points;
}
