import { expose } from "comlink";

import init, { INodeDetailInfo, IResult, SnapshotAnalysis } from "@wasm";
import { IFilterCondition, IThreadAPI, WorkerLogEvent } from "@/types";
import { I18n } from "@/i18n";

(self as any).Log = {
  set_msg: (msg: keyof I18n, params?: string[]) => {
    self.postMessage(new WorkerLogEvent(msg, params));
  },
};

class WorkerIns implements IThreadAPI {
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
    return this.analysis?.get_graph_info(cond) as IResult;
  }

  public async getNode(id: string) {
    return this.analysis?.get_node_detail_info(id) as INodeDetailInfo;
  }
}

expose(new WorkerIns());
