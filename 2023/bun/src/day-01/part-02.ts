const stringToNumber = (value: string) => {
  return value.split('').reduce(
    (acc, char) => {
      const nextStr = acc + char;
      switch (true) {
        case nextStr.includes('one'):
          return nextStr.replace('one', '1');
        case nextStr.includes('two'):
          return nextStr.replace('two', '2');
        case nextStr.includes('three'):
          return nextStr.replace('three', '3');
        case nextStr.includes('four'):
          return nextStr.replace('four', '4');
        case nextStr.includes('five'):
          return nextStr.replace('five', '5');
        case nextStr.includes('six'):
          return nextStr.replace('six', '6');
        case nextStr.includes('seven'):
          return nextStr.replace('seven', '7');
        case nextStr.includes('eight'):
          return nextStr.replace('eight', '8');
        case nextStr.includes('nine'):
          return nextStr.replace('nine', '9');
        default:
          return nextStr;
      }
    },
    value.substring(0, 3)
  );
};

const processLine = (line: string) => {
  const numbers = line.split('').filter((char) => !Number.isNaN(Number(char)));
  if (!numbers.length) return 0;

  const [first, ...rest] = numbers;

  const base = Number(first) * 10;

  if (!rest.length) return base + Number(first);

  return base + Number(rest.at(-1));
};

export const process = (value: string) => {
  return value.split('\n').reduce((acc, line) => {
    return acc + processLine(stringToNumber(line));
  }, 0);
};
