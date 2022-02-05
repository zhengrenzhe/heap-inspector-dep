import { singleton } from "tsyringe";

import { CompareMode, IFilterCondition } from "@/types";
import { inject, toJSON } from "@/util";
import { LogService, ThreadService } from "@/service";
import { action, makeObservable, observable } from "mobx";
import { transfer } from "comlink";

@singleton()
export class ParserService {
  constructor() {
    void this.threadService.initThread();
  }

  public viewModel = new ViewModel();

  @inject()
  private logService!: LogService;

  @inject()
  private threadService!: ThreadService;

  private get currentThread() {
    return this.threadService.currentThread;
  }

  public fromBuffer(buffer: ArrayBuffer) {
    void this.currentThread.parseData(transfer(buffer, [buffer]));
  }

  public async getGraphByFilter() {
    return await this.currentThread.getGraph(toJSON(this.viewModel.filter));
  }

  public async getNodeInfo(nodeId: number) {
    return await this.currentThread.getNode(nodeId);
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
