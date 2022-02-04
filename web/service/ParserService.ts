import { singleton } from "tsyringe";

import {
  BaseWorkerEvent,
  WorkerEventName,
  WorkerLogEvent,
  CompareMode,
  IFilterCondition,
  WorkerGetGraphEvent,
  WorkerReturnGraphEvent,
  WorkerGetNodeEvent,
} from "@/types";
import { inject, toJSON } from "@/util";
import { LogService, RenderService } from "@/service";
import { action, makeObservable, observable } from "mobx";

@singleton()
export class ParserService {
  constructor() {
    this.worker = new Worker(new URL("./worker.js", location.href));
    this.worker.addEventListener("message", this.onWorkerMessage);
  }

  private worker: Worker;

  public viewModel = new ViewModel();

  @inject()
  private renderService!: RenderService;

  @inject()
  private logService!: LogService;

  private onWorkerMessage = (e: MessageEvent<BaseWorkerEvent>) => {
    switch (e.data.name) {
      case WorkerEventName.Inited:
        return this.onWorkerInited();
      case WorkerEventName.Log:
        return this.onWorkerLog(e.data as WorkerLogEvent);
      case WorkerEventName.ReturnGraph:
        return this.onWorkerGraphReturn(e.data as WorkerReturnGraphEvent);
    }
  };

  private onWorkerInited() {
    this.logService.setMsg("worker-loaded");
  }

  private onWorkerLog(data: WorkerLogEvent) {
    if (data.message.length === 1) {
      return this.logService.setMsg(data.message[0] as any);
    }
    if (data.message.length === 2) {
      return this.logService.setMsg2(data.message[0] as any, data.message[1]);
    }
  }

  private onWorkerGraphReturn(data: WorkerReturnGraphEvent) {
    this.renderService.render(data.graph);
  }

  public fromBuffer(buffer: Uint8Array) {
    this.worker.postMessage(buffer.buffer, [buffer.buffer]);
  }

  public getGraphByFilter() {
    this.worker.postMessage(
      toJSON(new WorkerGetGraphEvent(this.viewModel.filter))
    );
  }

  public getNodeInfo(nodeId: string) {
    this.worker.postMessage(new WorkerGetNodeEvent(nodeId));
  }
}

class ViewModel {
  constructor() {
    makeObservable(this);
  }

  @observable
  public filter: IFilterCondition = {
    constructor_name: "",
    self_size: 0,
    self_size_compare_mode: CompareMode.MoreThan,
    retain_size: 0,
    retain_size_compare_mode: CompareMode.MoreThan,
    reference_depth: 0,
    reference_depth_compare_mode: CompareMode.MoreThan,
    nodes_limit: 10000,
    ignore_system_node: true,
  };

  @action
  public setFilter<T extends keyof IFilterCondition>(
    key: T,
    value: IFilterCondition[T]
  ) {
    this.filter[key] = value;
  }
}
