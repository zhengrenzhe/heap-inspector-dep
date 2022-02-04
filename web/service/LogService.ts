import { singleton } from "tsyringe";
import { action, makeObservable, observable } from "mobx";

import { I18n, i18n } from "@/i18n";

@singleton()
export class LogService {
  public viewModel = new ViewModel();

  public setMsg(msg: keyof I18n) {
    this.viewModel.setMsg(i18n(msg));
  }

  public setMsg2(m1: keyof I18n, m2: string) {
    this.viewModel.setMsg(`${i18n(m1)} ${m2}`);
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
    this.msg = msg;
  }
}
