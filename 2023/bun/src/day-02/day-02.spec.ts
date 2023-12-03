import { describe, expect, it } from 'bun:test';
import Bun from 'bun';
import { Cube } from './game.ts';
import { getFilePath } from '../utils.ts';

describe('day-02', () => {
  describe('part-01', async () => {
    const bag = [
      { color: 'red', count: 12 },
      { color: 'green', count: 13 },
      { color: 'blue', count: 14 },
    ] satisfies Cube[];

    const { process } = await import('./part-01.ts');
    it('should return 8', () => {
      const input = `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`;

      expect(process(input, bag)).toEqual(8);
    });

    it('should return 2727', async () => {
      const input = await Bun.file(getFilePath('./day-02/input_01.txt')).text();
      expect(process(input.trim(), bag)).toEqual(2727);
    });
  });

  describe('part-02', async () => {
    const { process } = await import('./part-02.ts');

    it('should return 2286', () => {
      const input = `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
      Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
      Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
      Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
      Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`;

      expect(process(input)).toEqual(2286);
    });

    it('should return 56580', async () => {
      const input = await Bun.file(getFilePath('./day-02/input_02.txt')).text();
      expect(process(input.trim())).toEqual(56580);
    });
  });
});
