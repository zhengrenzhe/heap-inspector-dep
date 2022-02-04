import init, { SnapshotAnalysis } from "../wasm/pkg";
import {
  BaseWorkerEvent,
  WorkerEventName,
  WorkerGetGraphEvent,
  WorkerInitedEvent,
  WorkerLogEvent,
  WorkerReturnGraphEvent,
} from "@/types";

let Analysis: SnapshotAnalysis;

(async () => {
  await init();
  self.postMessage(new WorkerInitedEvent());
})();

(self as any).Log = {
  set_msg: (msg: string) => {
    self.postMessage(new WorkerLogEvent([msg]));
  },
  set_msg2: (msg1: string, msg2: string) => {
    self.postMessage(new WorkerLogEvent([msg1, msg2]));
  },
};

self.addEventListener("message", (e: MessageEvent<BaseWorkerEvent>) => {
  if (e.data instanceof ArrayBuffer) {
    Analysis = new SnapshotAnalysis(new Uint8Array(e.data));
    return;
  }
  if (e.data.name === WorkerEventName.GetGraph) {
    const graph = Analysis.get_graph_info((e.data as WorkerGetGraphEvent).cond);
    self.postMessage(new WorkerReturnGraphEvent(graph));
    return;
  }
});
