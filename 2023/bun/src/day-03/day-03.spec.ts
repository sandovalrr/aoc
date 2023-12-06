import { describe, expect, it } from 'bun:test';
import { getFilePath } from '../utils';

describe('day-03', () => {
  describe('part-01', async () => {
    const { process } = await import('./part-01');

    it('should return 4361', () => {
      const input = `467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..`;

      expect(process(input)).toEqual(4361);
    });

    it('should return 527369', async () => {
      const input = await Bun.file(getFilePath('./day-03/input_01.txt')).text();
      expect(process(input.trim())).toEqual(527369);
    });
  });
});
