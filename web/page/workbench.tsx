import { ActionIcon, ColorScheme } from "@mantine/core";
import React, { Component } from "react";
import { observer } from "mobx-react";
import { MdDarkMode, MdLightMode } from "react-icons/md";

import { WorkbenchService } from "@web/page/workbench.service";
import { __, inject } from "@web/common";
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
        <ActionIcon
          c={this.isDark ? "#F8F9FA" : "#101113"}
          radius="md"
          size="md"
          color="teal"
          variant="filled"
          fz={18}
          style={{ cursor: "pointer" }}
          onClick={this.props.toggleColorScheme}
          title={this.isDark ? __("light_mode") : __("dark_mode")}
        >
          {this.isDark ? <MdLightMode /> : <MdDarkMode />}
        </ActionIcon>
      </div>
    );
  }
}
