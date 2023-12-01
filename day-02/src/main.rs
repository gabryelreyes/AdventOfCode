fn main() {
    let input1 = include_str!("../input1.txt");
    let input2 = include_str!("../input2.txt");
    let official_input = include_str!("../input_day-2.txt");

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

pub struct Game {
    m_id: u32,
    m_red: u32,
    m_green: u32,
    m_blue: u32,
}

impl Game {
    pub fn set_id(&mut self, id: u32) {
        self.m_id = id;
    }

    pub fn add_red(&mut self, value: u32) {
        self.m_red += value;
    }

    pub fn add_green(&mut self, value: u32) {
        self.m_green += value;
    }

    pub fn add_blue(&mut self, value: u32) {
        self.m_blue += value;
    }

    pub fn add_max_red(&mut self, value: u32) {
        if value > self.m_red {
            self.m_red = value;
        }
    }

    pub fn add_max_green(&mut self, value: u32) {
        if value > self.m_green {
            self.m_green = value;
        }
    }

    pub fn add_max_blue(&mut self, value: u32) {
        if value > self.m_blue {
            self.m_blue = value;
        }
    }

    pub fn print(&mut self) {
        println!(
            "ID: {:>3}, R: {:>3}, G: {:>3}, B: {:>3}",
            self.m_id, self.m_red, self.m_green, self.m_blue
        );
    }

    pub fn check(&mut self, red_limit: u32, green_limit: u32, blue_limit: u32) -> bool {
        let mut is_valid: bool = false;

        if self.m_red > red_limit {
            // println!("Red is too big! {} > {}", self.m_red, red_limit);
        } else if self.m_green > green_limit {
            // println!("Green is too big! {} > {}", self.m_green, green_limit);
        } else if self.m_blue > blue_limit {
            // println!("Blue is too big! {} > {}", self.m_blue, blue_limit);
        } else {
            is_valid = true;
        }

        return is_valid;
    }
}

fn part1(input: &str) -> u32 {
    let lines = input.split("\n");
    let mut total: u32 = 0;

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    for line in lines {
        let line_parts: Vec<&str> = line.split(":").collect();
        let header = line_parts[0];
        let body = line_parts[1];
        let id = header.split_at(5).1.parse::<u32>().unwrap();

        let subsets = body.split(";");
        let mut is_possible = true;

        for subset in subsets {
            let mut game: Game = Game {
                m_id: 0,
                m_red: 0,
                m_green: 0,
                m_blue: 0,
            };

            let colors = subset.split(",");

            for color in colors {
                let trimmed_color = color.trim();
                let key_value_pair: Vec<&str> = trimmed_color.split_whitespace().collect();

                let key = key_value_pair[1];
                let value: u32 = key_value_pair[0].parse().unwrap();

                match key {
                    "red" => game.add_red(value),
                    "green" => game.add_green(value),
                    "blue" => game.add_blue(value),
                    _ => panic!(),
                }
            }

            if false == game.check(red_limit, green_limit, blue_limit) {
                is_possible = false;
            }
        }

        if true == is_possible {
            total += id;
        }
    }

    return total;
}

fn part2(input: &str) -> u32 {
    let lines = input.split("\n");
    let mut total: u32 = 0;

    for line in lines {
        let line_parts: Vec<&str> = line.split(":").collect();
        let body = line_parts[1];

        let mut game: Game = Game {
            m_id: 0,
            m_red: 0,
            m_green: 0,
            m_blue: 0,
        };

        let subsets = body.split(";");
        for subset in subsets {
            let mut sample: Game = Game {
                m_id: 0,
                m_red: 0,
                m_green: 0,
                m_blue: 0,
            };

            let colors = subset.split(",");

            for color in colors {
                let trimmed_color = color.trim();
                let key_value_pair: Vec<&str> = trimmed_color.split_whitespace().collect();

                let key = key_value_pair[1];
                let value: u32 = key_value_pair[0].parse().unwrap();

                match key {
                    "red" => sample.add_red(value),
                    "green" => sample.add_green(value),
                    "blue" => sample.add_blue(value),
                    _ => panic!(),
                }
            }

            game.add_max_red(sample.m_red);
            game.add_max_green(sample.m_green);
            game.add_max_blue(sample.m_blue);
        }

        let game_power = game.m_red * game.m_green * game.m_blue;
        total += game_power;
        game.print();
    }

    return total;
}
