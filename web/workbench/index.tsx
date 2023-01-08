import React, { Component } from "react";
import { WorkbenchService } from "@web/service";
import { observer } from "mobx-react";

import { __, inject } from "@web/common";
import { ProgressIndicator } from "@fluentui/react";

import "./style.less";

@observer
export class Workbench extends Component {
  @inject()
  private workbenchService: WorkbenchService;

  public override componentDidMount() {
    this.workbenchService.init();
  }

  public override render() {
    const { isReady } = this.workbenchService.viewModel;
    return (
      <div id="workbench">
        {isReady ? this.renderContent() : this.renderProgress()}
      </div>
    );
  }

  private renderContent() {
    return <div>X</div>;
  }

  private renderProgress() {
    return (
      <ProgressIndicator
        className="progress"
        description={<div style={{ textAlign: "center" }}>{__("loading")}</div>}
      />
    );
  }
}
