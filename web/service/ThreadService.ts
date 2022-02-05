import { singleton } from "tsyringe";
import { wrap } from "comlink";

import { IThread, IThreadAPI, WorkerEventName, WorkerLogEvent } from "@/types";
import { LogService } from "@/service/LogService";
import { inject } from "@/util";

@singleton()
export class ThreadService {
  private thread: IThread | undefined;
  private worker: Worker | undefined;

  @inject()
  private logService!: LogService;

  public async initThread() {
    this.logService.setMsg("creating-thread");

    this.worker = new Worker(new URL("./worker.js", location.href));
    this.thread = wrap<IThreadAPI>(this.worker);

    this.worker.addEventListener("message", (e) => {
      if (e.data.name === WorkerEventName.Log) {
        const data = e.data as WorkerLogEvent;
        this.logService.setMsg(data.message, data.params);
      }
    });

    await this.thread.init();

    this.logService.setMsg("creating-thread-done");
  }

  public get currentThread() {
    if (!this.thread) {
      throw new Error("thread not exist");
    }

    return this.thread;
  }
}
