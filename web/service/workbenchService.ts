import axios from "axios";
import { action, makeObservable, observable } from "mobx";

import { API, injectable, until } from "@web/common";
import { presetDarkPalettes, presetPalettes } from "@ant-design/colors";

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

  public get themeValue(): "dark" | "light" {
    if (this.viewModel.theme === "dark") return "dark";
    if (this.viewModel.theme === "light") return "light";
    if (window.matchMedia?.("(prefers-color-scheme: dark)").matches) {
      return "dark";
    }
    return "light";
  }

  public get themeColor() {
    if (this.themeValue === "dark") return presetDarkPalettes;
    return presetPalettes;
  }

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
