enum JSONState {
  Start,
  Object,
}

const numberMap: Record<string, boolean> = {
  '0': true,
  '1': true,
  '2': true,
  '3': true,
  '4': true,
  '5': true,
  '6': true,
  '7': true,
  '8': true,
  '9': true,
};

export function readNumberTo(chunk: string, start: number) {
  let index = start;
  const len = chunk.length;
  while (index < len) {
    const char = chunk[index];
    if (numberMap[char]) {
      index += 1;
      continue;
    }
    break;
  }
  return index - 1;
}

export class JSONParser {
  private stateStack: JSONState[] = [JSONState.Start];

  private valueStack: unknown[] = [];

  private get lastState() {
    return this.stateStack[this.stateStack.length - 1];
  }

  private set lastState(state: JSONState) {
    this.stateStack.push(state);
  }

  private popState() {
    this.stateStack.pop();
  }

  private get valueStackLength() {
    return this.valueStack.length;
  }

  private gotObjectKeyValue() {
    const value = this.valueStack.pop();
    const key = this.valueStack.pop() as string;
    const obj = this.valueStack.pop() as Record<string, never>;
    this.valueStack.push({ ...obj, [key]: value });
  }

  public parse(chunk: string) {
    const len = chunk.length;
    let index = 0;

    while (index < len) {
      const char = chunk[index];

      if (char === '{') {
        this.lastState = JSONState.Object;
        this.valueStack.push({});
        index += 1;
        continue;
      }

      if (this.lastState === JSONState.Object) {
        if (char === '"') {
          const start = index + 1;
          const end = chunk.indexOf('"', start);
          const key = chunk.substring(start, end);
          index = end + 1;
          this.valueStack.push(key);
          continue;
        }

        if (numberMap[char]) {
          const start = index;
          const end = readNumberTo(chunk, start);
          const value = chunk.substring(start, end + 1);
          index = end + 1;
          this.valueStack.push(parseInt(value));
          continue;
        }

        if (char === 't') {
          index += 4;
          this.valueStack.push(true);
          continue;
        }

        if (char === 'f') {
          index += 5;
          this.valueStack.push(false);
          continue;
        }

        if (char === 'n') {
          index += 4;
          this.valueStack.push(null);
          continue;
        }

        if (char === ':') {
          index += 1;
          continue;
        }

        if (char === ',') {
          this.gotObjectKeyValue();
          index += 1;
          continue;
        }

        if (char === '}') {
          if (this.valueStackLength > 2) {
            this.gotObjectKeyValue();
          }
          this.popState();
          index += 1;
          continue;
        }
      }

      index += 1;
    }

    console.log(this.valueStack);

    return this.valueStack.pop();
  }
}
