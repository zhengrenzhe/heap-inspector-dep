import { ColorScheme, UnstyledButton } from "@mantine/core";
import React, { Component } from "react";
import { observer } from "mobx-react";
import { MdDarkMode, MdLightMode } from "react-icons/md";

import { WorkbenchService } from "@web/page/workbench.service";
import { inject } from "@web/common";
import { Progress } from "@web/page/progress";
import { Canvas } from "@web/page/canvas";

import "./workbench.less";

interface IWorkbenchProps {
  cs: ColorScheme;
  toggleColorScheme: () => void;
}

@observer
export class Workbench extends Component<IWorkbenchProps> {
  @inject()
  private wbService: WorkbenchService;

  private get isDark() {
    return this.props.cs === "dark";
  }

  public override componentDidMount() {
    this.wbService.init();
  }

  public override render() {
    return (
      <div
        className="workbench"
        style={{ background: this.isDark ? "#101113" : "#F8F9FA" }}
      >
        {this.wbService.viewModel.isReady ? <Canvas /> : <Progress />}
        {this.renderTheme()}
      </div>
    );
  }

  private renderTheme() {
    return (
      <div className="theme-toggle">
        <UnstyledButton
          c={this.isDark ? "#F8F9FA" : "#101113"}
          fz={22}
          onClick={this.props.toggleColorScheme}
        >
          {this.isDark ? <MdLightMode /> : <MdDarkMode />}
        </UnstyledButton>
      </div>
    );
  }
}
