import { Remote } from "comlink";

import {
  INodeDetailInfo,
  IResult,
  IFilterCondition,
  ISameStringCondition,
  IEdgeDetailInfo,
} from "@wasm";

export interface IThreadAPI {
  init(): Promise<void>;

  parseData(buffer: ArrayBuffer): Promise<void>;

  getGraph(cond: IFilterCondition): Promise<IResult>;

  getNodeDetail(id: number): Promise<INodeDetailInfo>;

  getEdgeDetail(edge_index: number): Promise<IEdgeDetailInfo>;

  getSameStringValueNodes(cond: ISameStringCondition): Promise<IResult>;
}

export type IThread = Remote<IThreadAPI>;
