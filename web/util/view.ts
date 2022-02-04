export function toNumber(val: string) {
  const num = parseInt(val);
  if (isNaN(num)) return 0;
  return num;
}

export function toJSON(data: any) {
  return JSON.parse(JSON.stringify(data));
}
