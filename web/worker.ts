import init, { SnapshotAnalysis } from "@wasm";
import {
  BaseWorkerEvent,
  WorkerEventName,
  WorkerGetGraphEvent,
  WorkerInitedEvent,
  WorkerLogEvent,
  WorkerReturnGraphEvent,
} from "@/types";
import { I18n } from "@/i18n";

let Analysis: SnapshotAnalysis;

(async () => {
  await init();
  self.postMessage(new WorkerInitedEvent());
})();

(self as any).Log = {
  set_msg: (msg: keyof I18n, params?: string[]) => {
    self.postMessage(new WorkerLogEvent(msg, params));
  },
};

self.addEventListener("message", (e: MessageEvent<BaseWorkerEvent>) => {
  if ((e.data as any) instanceof ArrayBuffer) {
    Analysis = new SnapshotAnalysis(
      new Uint8Array(e.data as unknown as ArrayBuffer)
    );
    return;
  }
  if (e.data.name === WorkerEventName.GetGraph) {
    const graph = Analysis.get_graph_info((e.data as WorkerGetGraphEvent).cond);
    self.postMessage(new WorkerReturnGraphEvent(graph));
    return;
  }
});
