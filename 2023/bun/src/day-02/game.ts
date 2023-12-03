import { z } from 'zod';

const ColorSchema = z.enum(['red', 'green', 'blue']);
const CubeSchema = z.object({
  color: ColorSchema,
  count: z.coerce.number().positive(),
});

export type Color = z.infer<typeof ColorSchema>;

export type Cube = z.infer<typeof CubeSchema>;

export type Round = {
  cubes: Cube[];
};

export type Game = {
  id: number;
  rounds: Round[];
};

function processRound(unprocessedRounds: string) {
  return unprocessedRounds.split(';').map((round) => {
    const cubes = round.split(', ').map((cube) => {
      const [count, color] = cube.trim().split(' ');
      return CubeSchema.parse({ color, count });
    });

    return {
      cubes,
    } satisfies Round;
  });
}

function isRoundPossible(round: Round, bag: Cube[]) {
  return round.cubes.every((cube) => {
    const bagCube = bag.find((bagCube) => bagCube.color === cube.color);

    if (!bagCube) {
      return false;
    }

    return bagCube.count >= cube.count;
  });
}

export function buildGame(line: string) {
  // regex to get the id
  const regex = /Game (\d+):/;

  // get the id
  const id = z.coerce.number().parse(line.match(regex)?.[1]);
  const unprocessedRounds = line.replace(regex, '');

  const rounds = processRound(unprocessedRounds);

  return {
    id,
    rounds: rounds,
  } satisfies Game;
}

export function isGamePossible(game: Game, bag: Cube[]) {
  return game.rounds.every((round) => isRoundPossible(round, bag));
}

export function getGamePower(game: Game) {
  const cubeMap = game.rounds.reduce((acc, round) => {
    round.cubes.forEach((cube) => {
      const currentCount = acc.get(cube.color);
      if (!currentCount) {
        acc.set(cube.color, cube.count);
        return;
      }

      if (currentCount >= cube.count) {
        return;
      }

      acc.set(cube.color, cube.count);
    });
    return acc;
  }, new Map<Color, number>());

  return Array.from(cubeMap.values()).reduce((acc, count) => acc * count, 1);
}
