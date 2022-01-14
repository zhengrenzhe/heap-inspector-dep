import { action, configure, makeObservable } from "mobx";
import { observable } from "mobx";

configure({
  useProxies: "always",
  enforceActions: "always",
});

export enum CompareMode {
  LessThan = 0,
  MoreThan = 1,
}

export interface IFilter {
  constructor_name: string;
  self_size: number;
  retain_size: number;
  reference_depth: number;
  self_size_compare_mode: CompareMode;
  retain_size_compare_mode: CompareMode;
  reference_depth_compare_mode: CompareMode;
}

export class ViewModel {
  constructor() {
    makeObservable(this);
  }

  @observable
  public filter: IFilter = {
    constructor_name: "",
    self_size: 0,
    self_size_compare_mode: CompareMode.MoreThan,
    retain_size: 0,
    retain_size_compare_mode: CompareMode.MoreThan,
    reference_depth: 0,
    reference_depth_compare_mode: CompareMode.MoreThan,
  };

  @action
  public setFilter<T extends keyof IFilter>(key: T, value: IFilter[T]) {
    this.filter[key] = value;
  }

  @observable
  public msg = "";

  @action
  public setMsg(msg: string) {
    this.msg = msg;
  }
}
