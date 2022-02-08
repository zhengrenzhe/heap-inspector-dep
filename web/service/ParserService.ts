import { singleton } from "tsyringe";
import { action, makeObservable, observable } from "mobx";
import { transfer } from "comlink";

import { CompareMode } from "@/types";
import { inject, toJSON } from "@/util";
import { LogService, ThreadService } from "@/service";
import { IFilterCondition, ISameStringCondition } from "@wasm";

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
    return await this.currentThread.getNodeDetail(nodeId);
  }

  public async getEdgeInfo(edgeIndex: number) {
    return await this.currentThread.getEdgeDetail(edgeIndex);
  }

  public async getSameStringValueNodes() {
    return await this.currentThread.getSameStringValueNodes(
      toJSON(this.viewModel.sameStringCond)
    );
  }
}

class ViewModel {
  constructor() {
    makeObservable(this);
  }

  @observable
  public filter: IFilterCondition = {
    filter_from: ["constructor_name"],
    filter_name: "",
    self_size: 0,
    self_size_compare_mode: CompareMode.MoreThan,
    retain_size: 0,
    retain_size_compare_mode: CompareMode.MoreThan,
    reference_depth: 0,
    reference_depth_compare_mode: CompareMode.MoreThan,
    nodes_limit: 1000,
    ignore_system_node: true,
  };

  @observable
  public sameStringCond: ISameStringCondition = {
    more_than_same_times: 2,
    minimum_string_len: 5,
    includes: [],
    excludes: [],
  };

  @action
  public setFilter<T extends keyof IFilterCondition>(
    key: T,
    value: IFilterCondition[T]
  ) {
    this.filter[key] = value;
  }

  @action
  public setSameStringCond<T extends keyof ISameStringCondition>(
    key: T,
    value: ISameStringCondition[T]
  ) {
    this.sameStringCond[key] = value;
  }
}
