import axios from "axios";
import { action, makeObservable, observable } from "mobx";

import { injectable, until } from "@web/common";

class ViewModel {
  @observable
  public isReady = false;

  constructor() {
    makeObservable(this);
  }
}

@injectable()
export class WorkbenchService {
  public viewModel = new ViewModel();

  public init() {
    void this.checkIsReady();
  }

  @action
  private async checkIsReady() {
    await until(
      () => axios.get("/api/is_ready"),
      (res) => res.data.is_ready
    );
    this.viewModel.isReady = true;
  }
}
