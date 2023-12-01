use regex::Regex;
use std::panic;

fn main() {
    let input1 = include_str!("../input1.txt");
    let input2 = include_str!("../input1.txt");

    println!("Part 1: {}", part1(input1));
    println!("Part 2: {}", part2(input2));
    println!("");
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n");

    let mut total: u32 = 0;

    for line in lines {
        let mut numbers: String = "".to_owned();
        let mut first_digit: bool = true;

        for character in line.chars() {
            if character.is_numeric() {
                if first_digit {
                    numbers.push(character);
                    first_digit = false;
                } else if numbers.len() == 1 {
                    numbers.push(character);
                } else {
                    numbers.remove(1);
                    numbers.push(character);
                }
            }
        }

        if numbers.len() == 1 {
            let first_digit: char = numbers.chars().nth(0).unwrap();
            numbers.push(first_digit);
        }

        let parsed_number: u32 = numbers.parse::<u32>().unwrap();
        total += parsed_number;
    }

    return total;
}

fn string_to_number(input: &str) -> char {
    let number: char;

    if input.len() == 1 {
        number = input.chars().nth(0).unwrap()
    } else {
        match input {
            "zero" => number = '0',
            "one" => number = '1',
            "two" => number = '2',
            "three" => number = '3',
            "four" => number = '4',
            "five" => number = '5',
            "six" => number = '6',
            "seven" => number = '7',
            "eight" => number = '8',
            "nine" => number = '9',
            _ => panic!(),
        }
    }

    return number;
}

fn find_first(line: &str) -> &str {
    let re = Regex::new(r"(?:zero|one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let words = re.find_at(line, 0).map(|m| m.as_str());

    return words.unwrap();
}

fn find_last(line: &str) -> &str {
    let re = Regex::new(r"(?:zero|one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    let mut last: Option<&str> = None;
    let mut starting_index = line.len();

    loop {
        let words = re.find_at(line, starting_index).map(|m| m.as_str());

        if words != None {
            last = words;
            break;
        } else {
            if starting_index == 0 {
                break;
            } else {
                starting_index = starting_index - 1;
            }
        }
    }

    return last.unwrap();
}

fn part2(input: &str) -> u32 {
    let lines = input.split("\n");
    let mut total: u32 = 0;

    for line in lines {
        let mut numbers: String = "".to_owned();

        let first = string_to_number(find_first(line));
        let last = string_to_number(find_last(line));

        numbers.push(first);
        numbers.push(last);

        total += numbers.parse::<u32>().unwrap();
    }

    return total;
}
