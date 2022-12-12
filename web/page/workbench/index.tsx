import React, { Component } from "react";

import { StateService } from "@web/page/workbench/state";
import { inject } from "@web/common";

import "./style.less";

export class Workbench extends Component {
  @inject()
  private stateService: StateService;

  public override render() {
    if (!this.stateService.currentState) return null;

    const View = this.stateService.currentState.view;
    return <View />;
  }
}
