pub mod schematic {

    pub enum SchematicType {
        Empty,
        Number(i32),
        Character(char),
    }

    #[derive(Debug)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
    }

    impl PartialEq for Coordinate {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl Coordinate {
        fn new(x: i32, y: i32) -> Coordinate {
            Coordinate { x, y }
        }

        pub fn get_adjacent_coordinates(&self) -> Vec<Coordinate> {
            // cartisian coordinates of adjacent positions
            let positions = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];

            let adjacent_coordinates = positions
                .iter()
                // create adjacent coordinates
                .map(|(x, y)| (self.x + x, self.y + y))
                // filter out negative coordinates
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                // convert to Coordinate
                .map(|(x, y)| Coordinate::new(x, y))
                // collect into a vector
                .collect::<Vec<Coordinate>>();

            adjacent_coordinates
        }

        fn is_left_of(&self, other: &Coordinate) -> bool {
            self.x + 1 == other.x
        }
    }

    #[derive(Debug)]
    pub struct Number {
        pub from: Coordinate,
        pub to: Coordinate,
        pub value: i32,
    }

    impl Number {
        pub fn is_x_coordinate_in_between(&self, coordinate: &Coordinate) -> bool {
            println!("Number: {:?}, coordinate: {:?}", self, coordinate,);
            self.from.y == coordinate.y && self.from.x <= coordinate.x && coordinate.x <= self.to.x
        }
    }

    #[derive(Debug)]
    pub struct Character {
        pub value: char,
        pub coordinate: Coordinate,
    }

    #[derive(Debug)]
    pub struct Schematic {
        pub numbers: Vec<Number>,
        pub characters: Vec<Character>,
    }

    impl Schematic {
        pub fn new(line: &str, y_coordinate: i32) -> Schematic {
            // create a vector of tuples containing the character and its coordinates
            let characters_with_coordinates =
                line.chars().enumerate().map(|(x_coordinate, character)| {
                    let matcher = match character {
                        '.' => ((x_coordinate as i32, y_coordinate), SchematicType::Empty),
                        character if character.is_digit(10) => (
                            (x_coordinate as i32, y_coordinate),
                            SchematicType::Number(character.to_digit(10).unwrap() as i32),
                        ),
                        character => (
                            (x_coordinate as i32, y_coordinate),
                            SchematicType::Character(character),
                        ),
                    };

                    matcher
                });

            let numbers = characters_with_coordinates
                .clone()
                .filter(|(_, num)| match num {
                    SchematicType::Number(_) => true,
                    _ => false,
                })
                .fold(Vec::new(), |mut acc, (coordinate, num)| {
                    let number = match num {
                        SchematicType::Number(number) => number,
                        _ => panic!("Not a number"),
                    };

                    if acc.len() == 0 {
                        return vec![Number {
                            from: Coordinate::new(coordinate.0, coordinate.1),
                            to: Coordinate::new(coordinate.0, coordinate.1),
                            value: number,
                        }];
                    }

                    match acc
                        .last()
                        .unwrap()
                        .to
                        .is_left_of(&Coordinate::new(coordinate.0, coordinate.1))
                    {
                        true => {
                            let last = acc.pop().unwrap();
                            let new_number = Number {
                                from: last.from,
                                to: Coordinate::new(coordinate.0, coordinate.1),
                                value: last.value * 10 + number,
                            };

                            acc.push(new_number);

                            acc
                        }
                        false => {
                            acc.push(Number {
                                from: Coordinate::new(coordinate.0, coordinate.1),
                                to: Coordinate::new(coordinate.0, coordinate.1),
                                value: number,
                            });

                            acc
                        }
                    }
                });

            let characters = characters_with_coordinates
                .clone()
                // filter out numbers an empty characters
                .filter(|(_, num)| match num {
                    SchematicType::Character(_) => true,
                    _ => false,
                })
                // convert to Character struct
                .map(|(coordinate, character)| {
                    let coordinate = Coordinate::new(coordinate.0, coordinate.1);
                    let character = match character {
                        SchematicType::Character(character) => character,
                        _ => panic!("Not a character"),
                    };

                    Character {
                        value: character,
                        coordinate,
                    }
                })
                .collect::<Vec<Character>>();

            Schematic {
                numbers,
                characters,
            }
        }
    }
}
