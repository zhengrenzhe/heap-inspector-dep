import { singleton } from "tsyringe";
import { action, makeObservable, observable } from "mobx";

import { I18n, i18n } from "@/i18n";

@singleton()
export class LogService {
  public viewModel = new ViewModel();

  public setMsg(msg: keyof I18n, params?: string[]) {
    this.viewModel.setMsg(i18n(msg, params));
  }
}

class ViewModel {
  constructor() {
    makeObservable(this);
  }

  @observable
  public msg = "";

  @action
  public setMsg(msg: string) {
    console.log(msg);
    this.msg = msg;
  }
}
