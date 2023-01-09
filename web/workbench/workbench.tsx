import React, { Component } from "react";
import { observer } from "mobx-react";
import LoaderInline from "@jetbrains/ring-ui/dist/loader-inline/loader-inline";
import { BrowserRouter } from "react-router-dom";

import { WorkbenchService } from "@web/service";
import { inject } from "@web/common";
import { Sidebar } from "@web/workbench/sidebar";
import { Main } from "@web/workbench/main";

import "@jetbrains/ring-ui/dist/style.css";

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
    return (
      <>
        <BrowserRouter>
          <Sidebar />
          <Main />
        </BrowserRouter>
      </>
    );
  }

  private renderProgress() {
    return <LoaderInline className="progress" />;
  }
}
