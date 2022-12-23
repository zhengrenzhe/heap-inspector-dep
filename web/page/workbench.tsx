import React, { Component } from "react";
import { observer } from "mobx-react";

import { WorkbenchService } from "@web/page/workbench.service";
import { inject } from "@web/common";
import { Progress } from "@web/page/progress";
import { Canvas } from "@web/page/canvas";

import "./workbench.less";

@observer
export class Workbench extends Component {
  @inject()
  private wbService: WorkbenchService;

  public override componentDidMount() {
    this.wbService.init();
  }

  public override render() {
    return (
      <div className="workbench">
        {this.wbService.viewModel.isReady ? <Canvas /> : <Progress />}
      </div>
    );
  }
}
