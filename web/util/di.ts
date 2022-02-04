import { container } from "tsyringe";

export function inject() {
  return (target: any, name: any): any => {
    const type = Reflect.getMetadata("design:type", target, name);
    const cls = container.resolve(type);

    if (!cls) {
      throw new Error(`${type} not found`);
    }

    const descriptor = {
      value: cls,
      enumerable: true,
      configurable: true,
    };

    Object.defineProperty(target, name, descriptor);
  };
}
