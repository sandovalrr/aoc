import { describe, expect, it } from 'bun:test';
import { getFilePath } from '../utils';

describe('day-04', () => {
  describe('part-01', async () => {
    const { process } = await import('./part-01.ts');

    it('should return 13', () => {
      const input = `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11`;
      expect(process(input)).toEqual(13);
    });

    it('should return 15268', async () => {
      const input = await Bun.file(getFilePath('./day-04/input_01.txt')).text();
      expect(process(input.trim())).toEqual(15268);
    });
  });

  describe('part-02', async () => {
    const { process } = await import('./part-02.ts');

    it('should return 30', () => {
      const input = `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11`;
      expect(process(input)).toEqual(30);
    });

    it('should return 6283755', async () => {
      const input = await Bun.file(getFilePath('./day-04/input_02.txt')).text();
      expect(process(input.trim())).toEqual(6283755);
    });
  });
});
