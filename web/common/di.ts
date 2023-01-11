/* eslint-disable */
import { Constructable, Container, Service } from "typedi";

export const injectable = Service;

export function inject() {
  return (target: any, name: any): any => {
    const type = Reflect.getMetadata("design:type", target, name);
    const cls = Container.get(type);

    if (!cls) {
      throw new Error(`${type} not found`);
    }

    const descriptor: PropertyDescriptor = {
      enumerable: true,
      configurable: true,
      get() {
        return cls;
      },
    };

    Object.defineProperty(target, name, descriptor);
  };
}

export function useService<T extends abstract new (...args: any) => any>(
  type: T
): InstanceType<T> {
  return Container.get(type as any);
}

export function contribution() {
  return function (target: any) {
    Reflect.defineMetadata(target.name, "", target);
  };
}

export function contributionImplement() {
  return function <T>(target: T) {
    const keys: string[] = Reflect.getMetadataKeys(target as any) ?? [];
    keys.forEach((key) => {
      Container.set({
        id: key,
        type: target as unknown as Constructable<T>,
        factory: undefined,
        multiple: true,
      });
    });
  };
}

export function getContributions(obj: any) {
  return function (target: any, propertyKey: string) {
    Object.defineProperty(target, propertyKey, {
      get() {
        return Container.getMany(obj.name);
      },
    });
  };
}

export function useContributions<T extends abstract new (...args: any) => any>(
  obj: T
): InstanceType<T>[] {
  return Container.getMany((obj as any).name);
}
