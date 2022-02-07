export function toJSON<T>(data: T): T {
  return JSON.parse(JSON.stringify(data));
}
