import { I18n } from "@/i18n";

export const filter_from: (keyof I18n)[] = [
  "constructor_name",
  "closure_name",
  "string_value",
];

export enum CompareMode {
  LessThan = 0,
  MoreThan = 1,
}
