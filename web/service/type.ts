export interface INode {
  id: string;
  node_type_index: number;
  name_index: number;
  self_size: number;
  edge_count: number;
}

export interface IEdge {
  source: string;
  target: string;
  edge_type_index: number;
}

export interface ISearchResult {
  edges: IEdge[];
  nodes: INode[];
}
