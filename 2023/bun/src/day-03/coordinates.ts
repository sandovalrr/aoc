import { z } from 'zod';

export type Coordinate = [number, number];
export type SchematicNumber = {
  from: Coordinate;
  to: Coordinate;
  value: number;
};

export type SchematicLine = {
  characters: Coordinate[];
  numbers: SchematicNumber[];
};
const OMITED_SYMBOL = '.';

const directions = [
  [-1, -1], // top left
  [-1, 0], // top
  [-1, 1], // top right

  [0, -1], // left
  [0, 1], // right

  [1, -1], // bottom left
  [1, 0], // bottom
  [1, 1], // bottom right
] satisfies Coordinate[];

export const getXCoordinatesInBetween = (from: Coordinate, to: Coordinate) => {
  if (from[1] !== to[1]) {
    throw new Error('Coordinates must be on the y axis');
  }

  const [x1, x2] = [from[0], to[0]];

  const length = x2 - x1 + 1;

  return Array.from(
    { length },
    (_, index) => [x1 + index, from[1]] satisfies Coordinate
  );
};

export const getAdjacentCoordinates = (coordinate: Coordinate) => {
  const [x, y] = coordinate;

  return (
    directions
      .map(([dx, dy]) => [x + dx, y + dy])
      // filter out negative coordinates
      .filter(([x, y]) => x >= 0 && y >= 0)
  );
};

type LineState = {
  previousValue?: string;
  line: SchematicLine;
};

export function getSchematicLine(characters: string[], yCoordinate: number) {
  const initialState: Readonly<LineState> = {
    line: {
      characters: [],
      numbers: [],
    },
  };

  return characters.reduce((state, character, xCoordinate) => {
    const previousNumber = z.coerce.number().safeParse(state.previousValue);
    const currentNumber = z.coerce.number().safeParse(character);
    const isLastCharacter = xCoordinate === characters.length - 1;

    // Is a character
    if (!currentNumber.success) {
      // previous value is number
      if (previousNumber.success) {
        const fromCoordinate = [
          xCoordinate - previousNumber.data.toString().length,
          yCoordinate,
        ] satisfies Coordinate;
        const toCoordinate = [
          xCoordinate - 1,
          yCoordinate,
        ] satisfies Coordinate;

        state.line.numbers.push({
          from: fromCoordinate,
          to: toCoordinate,
          value: previousNumber.data,
        });
      }

      if (character !== OMITED_SYMBOL) {
        state.line.characters.push([
          xCoordinate,
          yCoordinate,
        ] satisfies Coordinate);
      }

      return {
        ...state,
        previousValue: undefined,
      };
    }

    if (currentNumber.success) {
      const value = previousNumber.success
        ? previousNumber.data * 10 + currentNumber.data
        : currentNumber.data;

      if (!isLastCharacter) {
        return {
          ...state,
          previousValue: value.toString(),
        };
      }

      const diff = previousNumber.success
        ? previousNumber.data.toString().length
        : 0;

      const fromCoordinate = [
        xCoordinate - diff,
        yCoordinate,
      ] satisfies Coordinate;

      const toCoordinate = [xCoordinate, yCoordinate] satisfies Coordinate;

      state.line.numbers.push({
        from: fromCoordinate,
        to: toCoordinate,
        value,
      });

      return state;
    }

    return state;
  }, initialState).line;
}
