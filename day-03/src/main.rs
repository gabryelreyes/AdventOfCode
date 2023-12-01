use std::fmt;

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

pub struct Symbol {
    m_row: usize,
    m_col: usize,
    m_prod: u32,
    m_parts: u32,
}

impl Symbol {
    pub fn append(&mut self, val: u32) -> u32 {
        if self.m_parts == 0 {
            self.m_prod = val;
            self.m_parts += 1;
            return 0;
        } else if self.m_parts == 1 {
            self.m_prod *= val;
            self.m_parts += 1;
            return 0;
        } else {
            self.m_prod = 0;
            self.m_parts += 1;
            return 0;
        }
    }
}

#[derive(Clone)]
pub struct Number {
    m_number_string: String,
    m_number: u32,
    m_row: usize,
    m_cols: Vec<usize>,
}

impl Number {
    pub fn append_character(&mut self, character: char, col: usize) {
        // println!("Appending {} from {},{}", character, self.m_row, col);
        self.m_number_string.push(character);
        self.m_cols.push(col);
    }

    pub fn parse(&mut self) {
        // println!("Parsing");
        if self.m_number_string.len() != 0 {
            // println!("{}", self.m_number_string);
            self.m_number = self.m_number_string.parse().unwrap();
        }
    }

    pub fn is_empty(&self) -> bool {
        // dbg!(self.m_number_string.len());
        self.m_number_string.len() == 0
    }

    pub fn clear(&mut self) {
        self.m_number_string = String::from("");
        self.m_number = 0;
        self.m_cols = Vec::new();
    }
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} in ({},{:?})", self.m_number, self.m_row, self.m_cols)
    }
}
impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.m_row, self.m_col)
    }
}

fn part1(input: &str) -> u32 {
    let schematic = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut partial_sum = 0;

    for (row, line) in schematic.iter().enumerate() {
        let mut found_number = Number {
            m_number_string: String::from(""),
            m_number: 0,
            m_row: row,
            m_cols: Vec::new(),
        };

        for (col, character) in line.iter().enumerate() {
            if (*character != '.') && (*character != '\r') && (*character != '\n') {
                match character {
                    '0' => found_number.append_character(character.clone(), col),
                    '1' => found_number.append_character(character.clone(), col),
                    '2' => found_number.append_character(character.clone(), col),
                    '3' => found_number.append_character(character.clone(), col),
                    '4' => found_number.append_character(character.clone(), col),
                    '5' => found_number.append_character(character.clone(), col),
                    '6' => found_number.append_character(character.clone(), col),
                    '7' => found_number.append_character(character.clone(), col),
                    '8' => found_number.append_character(character.clone(), col),
                    '9' => found_number.append_character(character.clone(), col),
                    _ => {
                        symbols.push(Symbol {
                            m_row: row.clone(),
                            m_col: col.clone(),
                            m_prod: 0,
                            m_parts: 0,
                        });
                        if found_number.is_empty() == false {
                            found_number.parse();
                            numbers.push(found_number.clone());
                            found_number.clear();
                        }
                    }
                }
            } else {
                if found_number.is_empty() == false {
                    found_number.parse();
                    numbers.push(found_number.clone());
                    found_number.clear();
                }
            }
        }

        if found_number.is_empty() == false {
            found_number.parse();
            numbers.push(found_number.clone());
            found_number.clear();
        }
    }

    for num in &numbers {
        let mut found = false;
        let mut coordinates: Vec<Symbol> = Vec::new();
        let last_row = schematic.len() - 1;
        let last_column = schematic[0].len() - 1;

        for col in &num.m_cols {
            if num.m_row > 0 {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() - 1,
                    m_col: col.clone(),
                    m_prod: 0,
                    m_parts: 0,
                })
            }
            if num.m_row < (schematic.len() - 1) {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() + 1,
                    m_col: col.clone(),
                    m_prod: 0,
                    m_parts: 0,
                })
            }
        }

        if num.m_cols[0] != 0 {
            if num.m_row != 0 {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() - 1,
                    m_col: num.m_cols[0].clone() - 1,
                    m_prod: 0,
                    m_parts: 0,
                });
            }

            coordinates.push(Symbol {
                m_row: num.m_row.clone(),
                m_col: num.m_cols[0].clone() - 1,
                m_prod: 0,
                m_parts: 0,
            });

            if num.m_row != last_row {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() + 1,
                    m_col: num.m_cols[0].clone() - 1,
                    m_prod: 0,
                    m_parts: 0,
                });
            }
        }

        let end_of_number_idx = num.m_cols.len() - 1;

        if num.m_cols[end_of_number_idx] != last_column {
            if num.m_row != 0 {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() - 1,
                    m_col: num.m_cols[end_of_number_idx].clone() + 1,
                    m_prod: 0,
                    m_parts: 0,
                });
            }

            coordinates.push(Symbol {
                m_row: num.m_row.clone(),
                m_col: num.m_cols[end_of_number_idx].clone() + 1,
                m_prod: 0,
                m_parts: 0,
            });

            if num.m_row != last_row {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() + 1,
                    m_col: num.m_cols[end_of_number_idx].clone() + 1,
                    m_prod: 0,
                    m_parts: 0,
                });
            }
        }

        for coord in &coordinates {
            for symb in &symbols {
                if (coord.m_row == symb.m_row) && (coord.m_col == symb.m_col) {
                    found = true;
                    break;
                }
            }
        }

        if true == found {
            partial_sum += num.m_number.clone();
        }
    }

    return partial_sum;
}

fn part2(input: &str) -> u32 {
    let schematic = input
        .split("\n")
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut partial_sum = 0;

    for (row, line) in schematic.iter().enumerate() {
        let mut found_number = Number {
            m_number_string: String::from(""),
            m_number: 0,
            m_row: row,
            m_cols: Vec::new(),
        };

        for (col, character) in line.iter().enumerate() {
            if (*character != '.') && (*character != '\r') && (*character != '\n') {
                match character {
                    '0' => found_number.append_character(character.clone(), col),
                    '1' => found_number.append_character(character.clone(), col),
                    '2' => found_number.append_character(character.clone(), col),
                    '3' => found_number.append_character(character.clone(), col),
                    '4' => found_number.append_character(character.clone(), col),
                    '5' => found_number.append_character(character.clone(), col),
                    '6' => found_number.append_character(character.clone(), col),
                    '7' => found_number.append_character(character.clone(), col),
                    '8' => found_number.append_character(character.clone(), col),
                    '9' => found_number.append_character(character.clone(), col),
                    '*' => {
                        symbols.push(Symbol {
                            m_row: row.clone(),
                            m_col: col.clone(),
                            m_prod: 0,
                            m_parts: 0,
                        });
                        if found_number.is_empty() == false {
                            found_number.parse();
                            numbers.push(found_number.clone());
                            found_number.clear();
                        }
                    }
                    _ => {}
                }
            } else {
                if found_number.is_empty() == false {
                    found_number.parse();
                    numbers.push(found_number.clone());
                    found_number.clear();
                }
            }
        }

        if found_number.is_empty() == false {
            found_number.parse();
            numbers.push(found_number.clone());
            found_number.clear();
        }
    }

    dbg!(&symbols);

    for num in &numbers {
        let mut coordinates: Vec<Symbol> = Vec::new();
        let last_row = schematic.len() - 1;
        let last_column = schematic[0].len() - 1;

        for col in &num.m_cols {
            if num.m_row > 0 {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() - 1,
                    m_col: col.clone(),
                    m_prod: 0,
                    m_parts: 0,
                })
            }
            if num.m_row < (schematic.len() - 1) {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() + 1,
                    m_col: col.clone(),
                    m_prod: 0,
                    m_parts: 0,
                })
            }
        }

        if num.m_cols[0] != 0 {
            if num.m_row != 0 {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() - 1,
                    m_col: num.m_cols[0].clone() - 1,
                    m_prod: 0,
                    m_parts: 0,
                });
            }

            coordinates.push(Symbol {
                m_row: num.m_row.clone(),
                m_col: num.m_cols[0].clone() - 1,
                m_prod: 0,
                m_parts: 0,
            });

            if num.m_row != last_row {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() + 1,
                    m_col: num.m_cols[0].clone() - 1,
                    m_prod: 0,
                    m_parts: 0,
                });
            }
        }

        let end_of_number_idx = num.m_cols.len() - 1;

        if num.m_cols[end_of_number_idx] != last_column {
            if num.m_row != 0 {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() - 1,
                    m_col: num.m_cols[end_of_number_idx].clone() + 1,
                    m_prod: 0,
                    m_parts: 0,
                });
            }

            coordinates.push(Symbol {
                m_row: num.m_row.clone(),
                m_col: num.m_cols[end_of_number_idx].clone() + 1,
                m_prod: 0,
                m_parts: 0,
            });

            if num.m_row != last_row {
                coordinates.push(Symbol {
                    m_row: num.m_row.clone() + 1,
                    m_col: num.m_cols[end_of_number_idx].clone() + 1,
                    m_prod: 0,
                    m_parts: 0,
                });
            }
        }

        for coord in &coordinates {
            for symb in symbols.iter_mut() {
                if (coord.m_row == symb.m_row) && (coord.m_col == symb.m_col) {
                    partial_sum += symb.append(num.m_number.clone());
                    break;
                }
            }
        }
    }

    println!();

    for symb in symbols {
        if symb.m_parts == 2 {
            partial_sum += symb.m_prod;
        }
    }

    return partial_sum;
}
