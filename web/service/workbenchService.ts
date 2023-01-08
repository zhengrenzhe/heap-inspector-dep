import axios from "axios";
import { action, makeObservable, observable } from "mobx";

import { API, injectable, until } from "@web/common";

class ViewModel {
  @observable
  public isReady = false;

  constructor() {
    makeObservable(this);
  }

  @action
  public setReady() {
    this.isReady = true;
  }
}

@injectable()
export class WorkbenchService {
  public viewModel = new ViewModel();

  public init() {
    void this.checkIsReady();
  }

  private async checkIsReady() {
    await until(
      () => axios.get(API.is_ready),
      (res) => res.data.is_ready
    );
    this.viewModel.setReady();
  }
}
