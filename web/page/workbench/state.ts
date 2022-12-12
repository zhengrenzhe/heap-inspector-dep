import { ComponentClass } from "react";
import { makeObservable, observable } from "mobx";

import { contribution, getContributions, injectable } from "@web/common";

export enum State {
  Progressing,
}

class StateModel {
  @observable
  public state: State = State.Progressing;

  constructor() {
    makeObservable(this);
  }
}

@contribution()
export abstract class StateViewContribution {
  public abstract state: State;

  public abstract view: ComponentClass;
}

@injectable()
export class StateService {
  private model = new StateModel();

  @getContributions(StateViewContribution)
  private stateContribution: StateViewContribution[];

  public get currentState() {
    return this.stateContribution.find((c) => c.state === this.model.state);
  }
}
