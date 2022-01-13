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

export interface IFilterCondition {
  constructor_name: string;
}
