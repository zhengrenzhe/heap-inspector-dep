export enum CompareMode {
  LessThan = 0,
  MoreThan = 1,
}

export interface IFilterCondition {
  constructor_name: string;
  self_size: number;
  retain_size: number;
  reference_depth: number;
  nodes_limit: number;
  self_size_compare_mode: CompareMode;
  retain_size_compare_mode: CompareMode;
  reference_depth_compare_mode: CompareMode;
  ignore_system_node: boolean;
}
