fn main() {
    let input1 = include_str!("../input1.txt");
    let input2 = include_str!("../input2.txt");
    let official_input = include_str!("../official_input.txt");
    let expected_output_1 = 6592;
    let expected_output_2 = 6839;
    let output_1_test = part1(input1);
    let output_2_test = part2(input2);

    // if (false == input1.is_empty()) && (true == input2.is_empty()) {
    //     if expected_output_1 != output_1_test {
    //         panic!(
    //             "Failed Part 1. Expected {}, Got {}",
    //             expected_output_1, output_1_test
    //         );
    //     } else {
    //         println!("Test 1 passed!");
    //         println!("Part 1 Official: {}", part1(official_input));
    //     }
    // }

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

// const DENOMINATIONS: [char; 13] = [
//     '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
// ];
const DENOMINATIONS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandTypes {
    HIGHCARD,
    ONE,
    TWO,
    THREE,
    FULLHOUSE,
    FOUR,
    FIVE,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    m_type: HandTypes,
    m_card_values: Vec<usize>,
    m_cards: String,
    m_ordered_cards: String,
    m_bid: usize,
}

impl Hand {
    pub fn new(cards: String, bid: usize) -> Self {
        let (card_values, htype) = Self::sort(cards.clone());
        let ordered_cards: String = card_values
            .iter()
            .map(|deno| DENOMINATIONS[*deno])
            .collect();

        return Hand {
            m_type: htype,
            m_card_values: card_values,
            m_cards: cards,
            m_ordered_cards: ordered_cards,
            m_bid: bid,
        };
    }

    pub fn sort(cards: String) -> (Vec<usize>, HandTypes) {
        let mut denomination_count: Vec<usize> = vec![0; DENOMINATIONS.len()];
        let mut ordered: Vec<usize> = Vec::new();
        let mut natural: Vec<usize> = Vec::new();

        for card in cards.chars() {
            let index: usize = DENOMINATIONS.iter().position(|&r| r == card).unwrap();
            denomination_count[index] += 1;
            natural.push(index);
        }

        for (index, deno) in denomination_count.iter().enumerate() {
            for _i in 0..*deno {
                ordered.push(index);
            }
        }

        if ordered.len() != 5 {
            panic!("Wrong ordered lengt!");
        }

        let max = denomination_count.iter().max().unwrap();
        let min = denomination_count
            .iter()
            .filter(|&num| *num != 0)
            .min()
            .unwrap();
        let two_pairs: bool = denomination_count
            .iter()
            .filter(|&num| *num == 2_usize)
            .collect::<Vec<&usize>>()
            .len()
            == 2;

        let number_of_jokers = denomination_count[0];

        let htype: HandTypes;

        // PART 2
        match max {
            1 => match number_of_jokers {
                1 => htype = HandTypes::ONE,
                _ => htype = HandTypes::HIGHCARD,
            },
            2 => {
                if true == two_pairs {
                    match number_of_jokers {
                        2 => htype = HandTypes::FOUR,
                        1 => htype = HandTypes::FULLHOUSE,
                        _ => htype = HandTypes::TWO,
                    }
                } else {
                    match number_of_jokers {
                        2 => htype = HandTypes::THREE,
                        1 => htype = HandTypes::THREE,
                        _ => htype = HandTypes::ONE,
                    }
                }
            }
            3 => {
                if *min == 2_usize {
                    match number_of_jokers {
                        3 => htype = HandTypes::FIVE,
                        2 => htype = HandTypes::FIVE,
                        _ => htype = HandTypes::FULLHOUSE,
                    }
                } else {
                    match number_of_jokers {
                        3 => htype = HandTypes::FOUR,
                        1 => htype = HandTypes::FOUR,
                        _ => htype = HandTypes::THREE,
                    }
                }
            }
            4 => match number_of_jokers {
                4 => htype = HandTypes::FIVE,
                1 => htype = HandTypes::FIVE,
                _ => htype = HandTypes::FOUR,
            },
            5 => htype = HandTypes::FIVE,
            _ => panic!(),
        }

        // PART 1
        // match max {
        //     1 => htype = HandTypes::HIGHCARD,
        //     2 => {
        //         if true == two_pairs {
        //             htype = HandTypes::TWO;
        //         } else {
        //             htype = HandTypes::ONE;
        //         }
        //     }
        //     3 => {
        //         if *min == 2_usize {
        //             htype = HandTypes::FULLHOUSE;
        //         } else {
        //             htype = HandTypes::THREE;
        //         }
        //     }
        //     4 => htype = HandTypes::FOUR,
        //     5 => htype = HandTypes::FIVE,
        //     _ => panic!(),
        // }

        return (natural, htype);
        // return (ordered, htype);
    }
}

fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut total_winnings = 0;
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        hands.push(Hand::new(parts[0].to_string(), parts[1].parse().unwrap()));
    }

    hands.sort();

    for (rank, hand) in hands.iter().enumerate() {
        total_winnings += (rank + 1) * hand.m_bid;
    }

    return total_winnings.try_into().unwrap();
}

fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut total_winnings = 0;
    let mut hands: Vec<Hand> = Vec::new();

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        hands.push(Hand::new(parts[0].to_string(), parts[1].parse().unwrap()));
    }

    hands.sort();

    dbg!(&hands);

    for (rank, hand) in hands.iter().enumerate() {
        println!("{} {}", hand.m_cards, hand.m_bid);
        total_winnings += (rank + 1) * hand.m_bid;
    }

    return total_winnings.try_into().unwrap();
}
