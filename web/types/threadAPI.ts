import { Remote } from "comlink";

import {
  INodeDetailInfo,
  IResult,
  IFilterCondition,
  ISameStringCondition,
} from "@wasm";

export interface IThreadAPI {
  init(): Promise<void>;

  parseData(buffer: ArrayBuffer): Promise<void>;

  getGraph(cond: IFilterCondition): Promise<IResult>;

  getNodeDetail(id: number): Promise<INodeDetailInfo>;

  getSameStringValueNodes(cond: ISameStringCondition): Promise<IResult>;
}

export type IThread = Remote<IThreadAPI>;
