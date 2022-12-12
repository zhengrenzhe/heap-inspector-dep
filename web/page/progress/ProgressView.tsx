import React, { Component } from "react";

import { Progress } from "@mantine/core";

import { State, StateViewContribution } from "@web/page/workbench/state";
import { contributionImplement } from "@web/common";

import "./style.less";

class ProgressView extends Component {
  public override render() {
    return (
      <div id="progress-view">
        <Progress
          id="progress-bar"
          radius="xl"
          size="xl"
          value={50}
          striped
          animate
        />
      </div>
    );
  }
}

@contributionImplement()
export class ProgressViewContribution extends StateViewContribution {
  public state = State.Progressing;

  public view = ProgressView;
}
