/**
--- Day 2: Cube Conundrum ---
You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
In game 1, three sets of cubes are revealed from the bag (and then put back again).
The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
*/

mod game {

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
        pub fn new(id: i32, raw_game: &str) -> Game {
            let rounds = raw_game
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
    }
}

pub fn process(value: &str, bag: Vec<game::Cube>) -> i32 {
    let possible_games = value
        .lines()
        .map(|line| {
            let raw_game = line.split(": ").collect::<Vec<&str>>();
            let id = raw_game[0].split(' ').collect::<Vec<&str>>()[1]
                .parse::<i32>()
                .unwrap();
            let game = game::Game::new(id, raw_game[1]);

            game
        })
        .map(|game| game.possible_games(&bag))
        .sum::<i32>();

    possible_games
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(
            process(
                input,
                vec![
                    game::Cube::new("12 red"),
                    game::Cube::new("13 green"),
                    game::Cube::new("14 blue"),
                ]
            ),
            8
        );
        assert_eq!(
            process(
                include_str!("../input_01.txt",),
                vec![
                    game::Cube::new("12 red"),
                    game::Cube::new("13 green"),
                    game::Cube::new("14 blue"),
                ]
            ),
            2727
        );
    }
}
