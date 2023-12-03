import { Cube, buildGame, getGamePower } from './game';

export const process = (value: string) => {
  return value
    .split('\n')
    .map((line) => buildGame(line))
    .reduce((acc, game) => {
      return acc + getGamePower(game);
    }, 0);
};
