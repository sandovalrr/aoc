export const getFilePath = (filename: string) =>
  [import.meta.dir, filename].join('/');
