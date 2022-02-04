import { singleton } from "tsyringe";

import { BaseWorkerEvent, WorkerEventName, WorkerLogEvent } from "@/types";
import { inject } from "@/util";
import { LogService } from "@/service";

@singleton()
export class ParserService {
  constructor() {
    this.worker = new Worker(new URL("./worker.js", location.href));
    this.worker.addEventListener("message", this.onWorkerEvent);
  }

  private worker: Worker;

  @inject()
  private logService!: LogService;

  private onWorkerEvent = (e: MessageEvent<BaseWorkerEvent>) => {
    switch (e.data.name) {
      case WorkerEventName.Inited:
        return this.onWorkerInited();
      case WorkerEventName.Log:
        return this.onWorkerLog(e.data as WorkerLogEvent);
    }
  };

  private onWorkerInited = () => {
    this.logService.setMsg("worker-loaded");
  };

  private onWorkerLog = (data: WorkerLogEvent) => {
    if (data.message.length === 1) {
      return this.logService.setMsg(data.message[0] as any);
    }
    if (data.message.length === 2) {
      return this.logService.setMsg2(data.message[0] as any, data.message[1]);
    }
  };

  public fromBuffer(buffer: Uint8Array) {
    this.worker.postMessage(buffer.buffer, [buffer.buffer]);
  }
}
