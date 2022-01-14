export function toNumber(val: string) {
  const num = parseInt(val);
  if (isNaN(num)) return 0;
  return num;
}
