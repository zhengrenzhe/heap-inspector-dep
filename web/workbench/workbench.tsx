import React, { Component } from "react";
import { observer } from "mobx-react";
import { BrowserRouter } from "react-router-dom";
import { LoadingOutlined } from "@ant-design/icons";
import { ConfigProvider, Layout, Spin, theme } from "antd";

import { WorkbenchService } from "@web/service";
import { inject } from "@web/common";
import { Sidebar } from "@web/workbench/sidebar";
import { Main } from "@web/workbench/main";

import "./style.less";

@observer
export class Workbench extends Component {
  @inject()
  private workbenchService: WorkbenchService;

  private get theme() {
    if (this.workbenchService.themeValue === "dark") return theme.darkAlgorithm;
    return theme.defaultAlgorithm;
  }

  public override componentDidMount() {
    this.workbenchService.init();
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", () => this.forceUpdate());
  }

  public override render() {
    const { isReady } = this.workbenchService.viewModel;
    return (
      <ConfigProvider theme={{ algorithm: this.theme }}>
        <Layout id="workbench">
          {isReady ? this.renderContent() : this.renderProgress()}
        </Layout>
      </ConfigProvider>
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
    return <Spin indicator={<LoadingOutlined spin />} className="progress" />;
  }
}
