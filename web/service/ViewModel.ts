import { action, configure, makeObservable } from "mobx";
import { observable } from "mobx";

import { CompareMode, IFilterCondition } from "./type";

configure({
  useProxies: "always",
  enforceActions: "always",
});

export class ViewModel {
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
  };

  @action
  public setFilter<T extends keyof IFilterCondition>(
    key: T,
    value: IFilterCondition[T]
  ) {
    this.filter[key] = value;
  }

  @observable
  public msg = "";

  @action
  public setMsg(msg: string) {
    this.msg = msg;
  }
}
