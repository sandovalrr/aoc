import { z } from 'zod';

type Card = {
  id: number;
  winningNumbers: number[];
  numbers: number[];
};

export const getWinningMatches = (card: Card) => {
  return card.numbers.filter((x) => card.winningNumbers.includes(x));
};

export const buildCardFromLine = (line: string) => {
  const regex = line.match(/Card\s+([0-9]+)/);

  const rawCards = line.split(':')[1].split('|');

  const winningNumbers = z
    .array(z.coerce.number())
    .parse(rawCards[0].split(' ').filter((x) => x !== ''));

  const numbers = z
    .array(z.coerce.number())
    .parse(rawCards[1].split(' ').filter((x) => x !== ''));

  return {
    id: z.coerce.number().parse(regex?.[1]),
    numbers,
    winningNumbers,
  } satisfies Card;
};

export function processScratchCards(cards: Card[]): Card[] {
  return Array.from(
    cards
      .reduce(
        (acc, card, index, cards) => {
          // Get the number of matches for the current card
          const matches = getWinningMatches(card).length;

          if (matches === 0) return acc;

          // Get the number of copies of the current card
          const currentNumberOfCopies = acc.get(card) ?? 0;

          // Get the cards that will be won by the current card
          const copies = cards.slice(index + 1, index + matches + 1);

          // Iterate over the cards that will be won by the current card
          copies.forEach((card) => {
            // Get the number of copies of the card that will be won by the current card
            const numberOfCopies = acc.get(card) ?? 0;

            // Add the number of copies of the card that will be won by the current card to the accumulator
            acc.set(card, numberOfCopies + currentNumberOfCopies);
          });

          return acc;
        },
        new Map(cards.map((card) => [card, 1]))
      )
      .entries()
  )
    .map(([card, count]) => {
      return Array.from({
        length: count,
      }).map(() => ({ ...card }));
    })
    .flat();
}
