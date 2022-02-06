import { I18n } from "@/i18n";

export enum CompareMode {
  LessThan = 0,
  MoreThan = 1,
}

export const filter_from: (keyof I18n)[] = [
  "constructor_name",
  "closure_name",
  "string_value",
];

export interface IFilterCondition {
  filter_from: string[];
  filter_name: string;
  self_size: number;
  retain_size: number;
  reference_depth: number;
  nodes_limit: number;
  self_size_compare_mode: CompareMode;
  retain_size_compare_mode: CompareMode;
  reference_depth_compare_mode: CompareMode;
  ignore_system_node: boolean;
}
