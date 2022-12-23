import { action, makeObservable, observable } from "mobx";

import { injectable } from "@web/common";

interface IFilter {
  filter_from: string[];
  filter_name: string;
}

class ViewModel {
  @observable
  public filter: IFilter = {
    filter_from: ["constructor_name"],
    filter_name: "",
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
