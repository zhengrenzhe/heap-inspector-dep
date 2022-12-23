import React, { Component } from "react";
import { observer } from "mobx-react";

import { WorkbenchService } from "@web/page/workbench.service";
import { inject } from "@web/common";
import { Progress } from "@web/page/progress";

import "./workbench.less";

@observer
export class Workbench extends Component {
  @inject()
  private wbService: WorkbenchService;

  public override componentDidMount() {
    this.wbService.init();
  }

  public override render() {
    if (this.wbService.viewModel.isReady) return null;
    return <Progress />;
  }
}
