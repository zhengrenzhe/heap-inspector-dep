export async function until<T>(
  fn: () => Promise<T>,
  checker: (r: T) => boolean,
  interval = 100
): Promise<void> {
  const r = await fn();
  if (checker(r)) {
    return;
  }
  await sleep(interval);
  return until(fn, checker);
}

export async function sleep(ms: number) {
  return new Promise<void>((r) => {
    setTimeout(() => {
      r();
    }, ms);
  });
}

export function cx(obj: Record<string, boolean>) {
  return Object.keys(obj)
    .filter((k) => obj[k])
    .join(" ");
}
