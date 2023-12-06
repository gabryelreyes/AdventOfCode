fn main() {
    let input1 = include_str!("../input1.txt");
    let input2 = include_str!("../input2.txt");
    let official_input = include_str!("../official_input.txt");
    let expected_output_1 = 288;
    let expected_output_2 = 71503;
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

fn distance_traveled(held_time: u64, race_duration: u64) -> u64 {
    let driving_time = race_duration - held_time;
    return driving_time * held_time;
}

fn part1(input: &str) -> u64 {
    let lines = input.split("\n").map(|line| line.trim());
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
    let mut ways_to_win: u64 = 1;

    for line in lines {
        let temp = line.split(":").collect::<Vec<&str>>()[1]
            .split_whitespace()
            .map(|number| number.parse::<u64>().unwrap());

        for number in temp {
            if line.contains("Time:") {
                times.push(number);
            } else {
                distances.push(number);
            }
        }
    }

    let races: Vec<(&u64, &u64)> = times.iter().zip(distances.iter()).collect();

    for race in races {
        let mut min_held_time: u64 = 0;
        let mut max_held_time: u64 = *race.0;

        loop {
            let distance = distance_traveled(min_held_time, *race.0);
            if distance > *race.1 {
                break;
            } else {
                min_held_time += 1;
            }
        }

        loop {
            let distance = distance_traveled(max_held_time, *race.0);
            if distance > *race.1 {
                break;
            } else {
                max_held_time -= 1;
            }
        }

        if max_held_time < min_held_time {
            panic!("This should not be possible");
        }

        let ways_to_win_race = max_held_time - min_held_time + 1;

        ways_to_win *= ways_to_win_race;
    }

    return ways_to_win;
}

fn part2(input: &str) -> u64 {
    let lines = input.lines();
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();
    let mut ways_to_win: u64 = 1;

    for line in lines {
        let number = line.split(":").collect::<Vec<&str>>()[1]
            .trim()
            .replace(" ", "")
            .parse::<u64>()
            .unwrap();

        if line.contains("Time:") {
            times.push(number);
        } else {
            distances.push(number);
        }
    }

    let races: Vec<(&u64, &u64)> = times.iter().zip(distances.iter()).collect();

    for race in races {
        let mut min_held_time: u64 = 0;
        let mut max_held_time: u64 = *race.0;

        loop {
            let distance = distance_traveled(min_held_time, *race.0);
            if distance > *race.1 {
                break;
            } else {
                min_held_time += 1;
            }
        }

        loop {
            let distance = distance_traveled(max_held_time, *race.0);
            if distance > *race.1 {
                break;
            } else {
                max_held_time -= 1;
            }
        }

        if max_held_time < min_held_time {
            panic!("This should not be possible");
        }

        let ways_to_win_race = max_held_time - min_held_time + 1;
        ways_to_win *= ways_to_win_race;
    }

    return ways_to_win;
}
