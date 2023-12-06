use crate::schematic::schematic::Schematic;

/**
 --- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?
 */

pub fn process(value: &str) -> i32 {
    let (characters, numbers) = value
        .lines()
        .enumerate()
        .map(|(y_coordinate, line)| {
            let schematic = Schematic::new(line, y_coordinate as i32);

            (schematic.characters, schematic.numbers)
        })
        .fold(
            (Vec::new(), Vec::new()),
            |mut acc, (characters, numbers)| {
                acc.0.extend(characters);
                acc.1.extend(numbers);

                acc
            },
        );

    let sum = characters
        .iter()
        .map(|character| {
            let adjacent_numbers = match character.value {
                '*' => numbers
                    .iter()
                    .filter(|num| {
                        character
                            .coordinate
                            .get_adjacent_coordinates()
                            .iter()
                            .any(|coordinate| num.is_x_coordinate_in_between(coordinate))
                    })
                    .collect::<Vec<_>>(),
                _ => vec![],
            };

            match adjacent_numbers.len() {
                2 => {
                    let product = adjacent_numbers.iter().fold(1, |acc, num| acc * num.value);

                    product
                }
                _ => 0,
            }
        })
        .sum();

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(process(input), 467835);
        assert_eq!(process(include_str!("../input_02.txt",)), 73074886);
    }
}
