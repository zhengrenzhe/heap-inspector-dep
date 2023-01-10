import axios from "axios";
import { action, makeObservable, observable } from "mobx";

import { API, injectable, until } from "@web/common";

export type ITheme = "dark" | "light" | "auto";

class ViewModel {
  @observable
  public isReady = false;

  @observable
  public theme: ITheme = (localStorage.getItem("theme") as ITheme) || "auto";

  constructor() {
    makeObservable(this);
  }

  @action
  public setReady() {
    this.isReady = true;
  }

  @action
  public setTheme(newTheme: ITheme) {
    this.theme = newTheme;
    localStorage.setItem("theme", newTheme);
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
