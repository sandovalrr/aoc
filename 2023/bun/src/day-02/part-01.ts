import { Cube, buildGame, isGamePossible } from './game';

export const process = (value: string, bag: Cube[]) => {
  return value
    .split('\n')
    .map((line) => buildGame(line))
    .reduce((acc, game) => {
      // Check if the game is possible
      if (!isGamePossible(game, bag)) return acc;
      // If it is, add the id to the accumulator
      return acc + game.id;
    }, 0);
};
