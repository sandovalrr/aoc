pub mod game {

    pub enum Color {
        Red,
        Green,
        Blue,
    }

    impl PartialEq for Color {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Color::Red, Color::Red) => true,
                (Color::Green, Color::Green) => true,
                (Color::Blue, Color::Blue) => true,
                _ => false,
            }
        }
    }

    pub struct Cube {
        // Color of the cube
        color: Color,
        // Number of cubes of the same color
        count: i32,
    }

    impl Cube {
        pub fn new(value: &str) -> Cube {
            let cube = value.split(' ').collect::<Vec<&str>>();

            let color = match cube[1] {
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                _ => panic!("Invalid color"),
            };

            let count = cube[0].parse::<i32>().unwrap();

            Cube { color, count }
        }
    }

    struct Round {
        // Cubes revealed in the round
        cubes: Vec<Cube>,
    }

    impl Round {
        fn new(raw_round: &str) -> Round {
            let cubes = raw_round
                .split(',')
                .map(|cube| Cube::new(cube.trim()))
                .collect::<Vec<Cube>>();

            Round { cubes }
        }

        fn is_possible(&self, bag: &Vec<Cube>) -> bool {
            self.cubes.iter().all(|cube| match cube.color {
                Color::Red => {
                    let red_cubes = bag.iter().find(|c| c.color == Color::Red).unwrap();
                    red_cubes.count >= cube.count
                }
                Color::Green => {
                    let green_cubes = bag.iter().find(|c| c.color == Color::Green).unwrap();
                    green_cubes.count >= cube.count
                }
                Color::Blue => {
                    let blue_cubes = bag.iter().find(|c| c.color == Color::Blue).unwrap();
                    blue_cubes.count >= cube.count
                }
            })
        }
    }

    pub struct Game {
        // Rounds of the game
        id: i32,
        rounds: Vec<Round>,
    }

    impl Game {
        pub fn new(row: &str) -> Game {
            let raw_game = row.split(": ").collect::<Vec<&str>>();
            let id = raw_game[0].split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();

            let rounds = raw_game[1]
                .split(';')
                .map(|raw_game| {
                    let round = Round::new(raw_game);
                    round
                })
                .collect::<Vec<Round>>();

            Game { rounds, id }
        }

        pub fn possible_games(&self, bag: &Vec<Cube>) -> i32 {
            // if all rounds are possible, then the game is possible return the total of the game
            match self.rounds.iter().all(|round| round.is_possible(bag)) {
                true => self.id,
                false => 0,
            }
        }

        pub fn power(&self) -> i32 {
            let mut red_cubes = 0;
            let mut green_cubes = 0;
            let mut blue_cubes = 0;

            for round in &self.rounds {
                for cube in &round.cubes {
                    match cube.color {
                        Color::Red => match red_cubes < cube.count {
                            true => red_cubes = cube.count,
                            false => (),
                        },
                        Color::Green => match green_cubes < cube.count {
                            true => green_cubes = cube.count,
                            false => (),
                        },
                        Color::Blue => match blue_cubes < cube.count {
                            true => blue_cubes = cube.count,
                            false => (),
                        },
                    }
                }
            }

            red_cubes * green_cubes * blue_cubes
        }
    }
}
