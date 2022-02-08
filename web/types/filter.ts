import { IFilterFrom } from "@wasm";

export const filter_from: IFilterFrom[] = [
  "constructor_name",
  "closure_name",
  "string_value",
];

export enum CompareMode {
  LessThan = 0,
  MoreThan = 1,
}
