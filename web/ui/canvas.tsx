import React, { Component } from "react";

import { SnapshotService } from "@/service";

import { FilterPanel } from "./FilterPanel";

export class Canvas extends Component {
  public componentDidMount() {
    SnapshotService.init();
  }

  public render() {
    return (
      <div className="canvas-root">
        <div className="panels">
          <FilterPanel />
        </div>
        <div id="canvas" />
      </div>
    );
  }
}
