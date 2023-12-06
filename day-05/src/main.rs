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

    // println!("Part 2 Official: {}", part2(official_input));
    // println!();
    // println!();
}

#[derive(Debug)]
pub struct AlmanacMap {
    m_source_name: String,
    m_destination_name: String,
    m_destination_start: Vec<i64>,
    m_source_start: Vec<i64>,
    m_range: Vec<i64>,
}

const BLANK_ALMANAC: AlmanacMap = AlmanacMap {
    m_source_name: String::new(),
    m_destination_name: String::new(),
    m_destination_start: Vec::new(),
    m_source_start: Vec::new(),
    m_range: Vec::new(),
};

fn digest_map(input: i64, map: &AlmanacMap) -> i64 {
    let mut value: i64 = input;

    for idx in 0..map.m_source_start.len() {
        let range_start = map.m_source_start[idx];
        let range_end = map.m_source_start[idx] + map.m_range[idx];
        if (input >= range_start) && (input < range_end) {
            value = map.m_destination_start[idx] - map.m_source_start[idx] + input;
            break;
        }
    }

    return value;
}

fn digest_map_range(input: (i64, i64), map: &AlmanacMap) -> Vec<(i64, i64)> {
    let mut output: Vec<(i64, i64)> = Vec::new();

    for idx in 0..map.m_source_start.len() {
        let map_range_start = map.m_source_start[idx];
        let map_range_end = map.m_source_start[idx] + map.m_range[idx];
        let input_range_start = input.0;
        let input_range_end = input.1;
        let mut value_start = input.0;
        let mut value_end = input.1;

        if input_range_start >= map_range_start {
            value_start =
                map.m_destination_start[idx] - map.m_source_start[idx] + input_range_start;

            if input_range_end < map_range_end {
                value_end =
                    map.m_destination_start[idx] - map.m_source_start[idx] + input_range_end;
            } else {
                let difference = input_range_end - map_range_end;
                value_end = map_range_end - 1;
                output.push((map_range_end, input_range_end));
            }
            output.push((value_start, value_end));

            break;
        }
    }

    return output;
}

fn part1(input: &str) -> i64 {
    let lines = input.split("\n");

    let mut maps_vector: Vec<AlmanacMap> = Vec::new();
    let mut idx: usize = 0;
    let mut seeds: Vec<i64> = Vec::new();

    for line in lines {
        if line.len() == 0 {
            // Blank space. Ignore it
        } else if line.contains("map:") {
            // Name of map
            let mut map: AlmanacMap = AlmanacMap { ..BLANK_ALMANAC };

            let categories = line.split_whitespace().collect::<Vec<&str>>()[0]
                .split("-")
                .collect::<Vec<&str>>();
            map.m_source_name = categories[0].to_string();
            map.m_destination_name = categories[2].to_string();

            if maps_vector.is_empty() == false {
                idx += 1;
            }

            maps_vector.push(map);
        } else if line.contains("seeds:") {
            let seeds_iter: Vec<i64> = line.split(":").collect::<Vec<&str>>()[1]
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();

            for seed in seeds_iter {
                seeds.push(seed);
            }
        } else {
            // Numbers of the map
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();

            if numbers.len() != 3 {
                panic!();
            }

            maps_vector[idx].m_destination_start.push(numbers[0]);
            maps_vector[idx].m_source_start.push(numbers[1]);
            maps_vector[idx].m_range.push(numbers[2]);
        }
    }

    let mut map_matrix: Vec<Vec<i64>> = vec![vec![0; seeds.len()]; maps_vector.len() + 1];

    for idx in 0..seeds.len() {
        map_matrix[0][idx] = seeds[idx];
    }

    for idx in 0..map_matrix.len() - 1 {
        for seed in 0..seeds.len() {
            map_matrix[idx + 1][seed] = digest_map(map_matrix[idx][seed], &maps_vector[idx])
        }
    }

    return *map_matrix[map_matrix.len() - 1].iter().min().unwrap();
}

fn part2(input: &str) -> i64 {
    let lines = input.split("\n");

    let mut maps_vector: Vec<AlmanacMap> = Vec::new();
    let mut idx: usize = 0;
    let mut seeds: Vec<(i64, i64)> = Vec::new();

    for line in lines {
        if line.len() == 0 {
            // Blank space. Ignore it
        } else if line.contains("map:") {
            // Name of map
            let mut map: AlmanacMap = AlmanacMap { ..BLANK_ALMANAC };

            let categories = line.split_whitespace().collect::<Vec<&str>>()[0]
                .split("-")
                .collect::<Vec<&str>>();
            map.m_source_name = categories[0].to_string();
            map.m_destination_name = categories[2].to_string();

            if maps_vector.is_empty() == false {
                idx += 1;
            }

            maps_vector.push(map);
        } else if line.contains("seeds:") {
            let seeds_iter: Vec<i64> = line.split(":").collect::<Vec<&str>>()[1]
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();

            for idx in (0..seeds_iter.len()).step_by(2) {
                let range_start = seeds_iter[idx];
                let range_end = seeds_iter[idx] + seeds_iter[idx + 1];

                seeds.push((range_start, range_end));
            }

            // =========================================
            // BAD IDEA AHEAD!

            // if 0 != (seeds_iter.len() % 2) {
            //     panic!();
            // }

            // for idx in (0..seeds_iter.len()).step_by(2) {
            //     let range_start = seeds_iter[idx];
            //     let range_end = seeds_iter[idx] + seeds_iter[idx + 1];

            //     for seed in range_start..range_end {
            //         seeds.push(seed);
            //     }
            // }
            // =========================================
        } else {
            // Numbers of the map
            let numbers: Vec<i64> = line
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect();

            if numbers.len() != 3 {
                panic!();
            }

            maps_vector[idx].m_destination_start.push(numbers[0]);
            maps_vector[idx].m_source_start.push(numbers[1]);
            maps_vector[idx].m_range.push(numbers[2]);
        }
    }

    let mut map_matrix: Vec<Vec<(i64, i64)>> = Vec::new();

    map_matrix.push(seeds);
    // dbg!(&map_matrix);

    for idx in 0..(maps_vector.len()) {
        // for idx in 0..1 {
        println!("{}", idx);
        let col = map_matrix[idx].clone();
        let mut mapped: Vec<(i64, i64)> = Vec::new();
        for row in col {
            let out = digest_map_range(row, &maps_vector[idx]);
            for tup in out {
                mapped.push(tup);
            }
        }
        dbg!(&mapped);
        map_matrix.push(mapped);
        dbg!(&map_matrix);
        println!();
    }

    // return *map_matrix[map_matrix.len() - 1].iter().min().unwrap();
    return 0;
}
