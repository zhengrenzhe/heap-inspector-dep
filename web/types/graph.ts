import { FilterCondition } from "@wasm";

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
  id: string;
  node_type: string;
  node_name: string;
  self_size: number;
  edge_count: number;
}

export const CompareMode = {
  LessThan: 0,
  MoreThan: 1,
};

export type IFilterCondition = Omit<FilterCondition, "free">;
