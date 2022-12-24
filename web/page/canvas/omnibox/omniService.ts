import { action, makeObservable, observable } from "mobx";

import { injectable } from "@web/common";

interface IFilter {
  filter_from: string[];
  filter_name: string;
  self_size_mode: string;
  self_size: number;
  retained_size_mode: string;
  retained_size: number;
  depth: number;
}

class ViewModel {
  @observable
  public filter: IFilter = {
    filter_from: ["constructor_name"],
    filter_name: "",
    self_size_mode: "more_than",
    self_size: 0,
    retained_size: 0,
    retained_size_mode: "more_than",
    depth: 0,
  };

  @observable
  public mode = "filter";

  constructor() {
    makeObservable(this);
  }

  @action
  public setFilter<T extends keyof IFilter>(key: T, value: IFilter[T]) {
    this.filter[key] = value;
  }

  @observable
  public setMode(mode: string) {
    this.mode = mode;
  }
}

@injectable()
export class OmniService {
  public viewModel = new ViewModel();
}
