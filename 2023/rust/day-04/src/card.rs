use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Card {
    pub id: i32,
    winning_numbers: Vec<i32>,
    numbers: Vec<i32>,
}

impl Card {
    pub fn new(line: &str) -> Card {
        let card_sections = line.split(':').collect::<Vec<&str>>();
        let id = card_sections[0]
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let raw_cards = card_sections[1].split(" | ").collect::<Vec<&str>>();

        let winning_numbers = raw_cards[0]
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let numbers = raw_cards[1]
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Card {
            id,
            winning_numbers,
            numbers,
        }
    }

    pub fn get_winning_matches(&self) -> Vec<i32> {
        self.numbers
            .iter()
            .filter(|x| self.winning_numbers.contains(x))
            .map(|x| *x)
            .collect::<Vec<i32>>()
    }
}
pub struct ScratchCards {
    cards: Vec<Card>,
    map: HashMap<i32, Card>,
}

impl ScratchCards {
    pub fn new(cards: Vec<Card>) -> ScratchCards {
        let mut map = HashMap::new();

        cards.iter().for_each(|card| {
            map.insert(card.id, card.clone());
        });

        ScratchCards { cards, map }
    }

    pub fn get_copies(&self) -> Vec<&Card> {
        let mut initial_map = HashMap::new();

        self.cards.iter().for_each(|card| {
            initial_map.insert(card.id, 1);
        });

        let result = self
            .cards
            .iter()
            .enumerate()
            .fold(initial_map, |mut map, (index, card)| {
                let matches = card.get_winning_matches();
                let start_of_slice = index + 1;
                let end_of_slice = start_of_slice + matches.len();

                match matches.len() {
                    0 => (),
                    _ => {
                        let current_number_of_copies = *map.entry(card.id).or_insert(0);

                        self.cards[start_of_slice..end_of_slice]
                            .iter()
                            .for_each(|card| {
                                let number_of_copies = map.entry(card.id).or_insert(0);
                                *number_of_copies += current_number_of_copies;
                            })
                    }
                }

                map
            })
            .iter()
            .map(|(card_id, count)| {
                let card = self.map.get(card_id).unwrap();
                println!("card: {}, count: {}", card_id, count);
                // copy card count times
                let mut copies = Vec::new();

                for _ in 0..*count {
                    copies.push(card);
                }

                copies
            })
            .flatten()
            .collect::<Vec<&Card>>();

        result
    }
}
