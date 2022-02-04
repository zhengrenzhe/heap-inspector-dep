import React, { Component, createRef } from "react";

import { FilterPanel } from "@/ui/parts/FilterPanel";
import { RenderService } from "@/service";
import { inject } from "@/util";

export class Canvas extends Component {
  @inject()
  private renderService!: RenderService;

  private rootRef = createRef<HTMLDivElement>();

  public componentDidMount() {
    this.renderService.init(this.rootRef.current!);
  }

  public render() {
    return (
      <div className="canvas-root">
        <div className="panels">
          <FilterPanel />
        </div>
        <div id="canvas" ref={this.rootRef} />
      </div>
    );
  }
}
