export interface INode {
  [key: string]: any;
  id: string;
}

export interface IEdge {
  [key: string]: any;
  source: string;
  target: string;
}

export interface ISearchResult {
  edges: IEdge[];
  nodes: INode[];
}

export interface INodeDetailInfo {
  node_id: string;
  node_type: string;
  node_name: string;
  self_size: number;
  edge_count: number;
}

export enum CompareMode {
  LessThan = 0,
  MoreThan = 1,
}

export interface IFilterCondition {
  constructor_name: string;
  self_size: number;
  retain_size: number;
  reference_depth: number;
  self_size_compare_mode: CompareMode;
  retain_size_compare_mode: CompareMode;
  reference_depth_compare_mode: CompareMode;
}
