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
    return acc + processLine(line);
  }, 0);
};
