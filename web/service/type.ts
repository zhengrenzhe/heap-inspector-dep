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
