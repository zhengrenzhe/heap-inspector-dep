import { expose } from "comlink";

import init, {
  IEdgeDetailInfo,
  IFilterCondition,
  INodeDetailInfo,
  IResult,
  ISameStringCondition,
  SnapshotAnalysis,
} from "@wasm";
import { IThreadAPI, WorkerLogEvent } from "@/types";
import { I18n } from "@/i18n";

(self as any).Log = {
  set_msg: (msg: keyof I18n) => {
    self.postMessage(new WorkerLogEvent(msg));
  },
  set_msg_1_number: (msg: keyof I18n, num1: number) => {
    self.postMessage(new WorkerLogEvent(msg, [num1.toString()]));
  },
  set_msg_2_number: (msg: keyof I18n, num1: number, num2: number) => {
    self.postMessage(
      new WorkerLogEvent(msg, [num1.toString(), num2.toString()])
    );
  },
};

class Thread implements IThreadAPI {
  private analysis: SnapshotAnalysis | undefined;

  public async init() {
    await init();
  }

  public async parseData(buffer: ArrayBuffer) {
    if (this.analysis) {
      this.analysis.free();
    }

    this.analysis = new SnapshotAnalysis(new Uint8Array(buffer));
  }

  public async getGraph(cond: IFilterCondition) {
    return this.analysis?.get_graph(cond) as IResult;
  }

  public async getNodeDetail(id: number) {
    return this.analysis?.get_node_detail(id) as INodeDetailInfo;
  }

  public async getEdgeDetail(edge_index: number) {
    return this.analysis?.get_edge_detail(edge_index) as IEdgeDetailInfo;
  }

  public async getSameStringValueNodes(cond: ISameStringCondition) {
    return this.analysis?.get_same_string_value_nodes(cond) as IResult;
  }
}

expose(new Thread());
