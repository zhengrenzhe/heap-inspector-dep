import { Remote } from "comlink";

import { IFilterCondition } from "@/types";
import { INodeDetailInfo, IResult } from "@wasm";

export interface IThreadAPI {
  init(): Promise<void>;

  parseData(buffer: ArrayBuffer): Promise<void>;

  getGraph(cond: IFilterCondition): Promise<IResult>;

  getNode(id: number): Promise<INodeDetailInfo>;
}

export type IThread = Remote<IThreadAPI>;
