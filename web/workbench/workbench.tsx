import { ActionIcon, ColorScheme } from "@mantine/core";
import React, { Component } from "react";
import { observer } from "mobx-react";
import { MdDarkMode, MdLightMode } from "react-icons/md";

import { __, inject } from "@web/common";
import { WorkbenchService } from "@web/workbench/workbench.service";
import { Progress } from "@web/workbench/progress";
import { Omnibox } from "@web/workbench/omnibox";

import "./workbench.less";
import { Canvas } from "@web/workbench/canvas/canvas";

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
        {this.renderTheme()}
        {this.wbService.viewModel.isReady ? (
          <>
            <Omnibox />
            <Canvas />
          </>
        ) : (
          <Progress />
        )}
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
