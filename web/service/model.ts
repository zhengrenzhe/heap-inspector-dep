import { action, configure, makeObservable } from "mobx";
import { observable } from "mobx";

configure({
  useProxies: "always",
  enforceActions: "always",
});

export class ViewModel {
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
