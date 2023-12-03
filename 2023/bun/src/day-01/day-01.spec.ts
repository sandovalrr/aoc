import { describe, expect, it } from 'bun:test';
import Bun from 'bun';
import { getFilePath } from '../utils.ts';

describe('day-01', () => {
  describe('part-01', async () => {
    const { process } = await import('./part-01.ts');

    it('should return 142', () => {
      const input = `1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet`;
      expect(process(input)).toEqual(142);
    });

    it('should return 55090', async () => {
      const input = await Bun.file(getFilePath('day-01/input_01.txt')).text();
      expect(process(input)).toEqual(55090);
    });
  });

  describe('part-02', async () => {
    const { process } = await import('./part-02.ts');

    it('should return 281', () => {
      const input = `two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen`;
      expect(process(input)).toEqual(281);
    });

    it('should return 54871', async () => {
      const input = await Bun.file(getFilePath('day-01/input_02.txt')).text();
      expect(process(input)).toEqual(54871);
    });
  });
});
