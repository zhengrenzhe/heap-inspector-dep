import init, { SnapshotAnalysis } from "@wasm";
import { BaseWorkerEvent, WorkerInitedEvent, WorkerLogEvent } from "@/types";

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
  console.log(e);
});
