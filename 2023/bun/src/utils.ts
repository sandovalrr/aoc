import { join } from 'node:path';

export const getFilePath = (filename: string) =>
  join(import.meta.dir, filename);
