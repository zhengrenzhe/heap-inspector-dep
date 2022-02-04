import React, { Component, createRef } from "react";

import { FilterPanel } from "@/ui/parts/FilterPanel";
import { ParserService, RenderService } from "@/service";
import { inject } from "@/util";

export class Canvas extends Component {
  @inject()
  private renderService!: RenderService;

  @inject()
  private parserService!: ParserService;

  private rootRef = createRef<HTMLDivElement>();

  public componentDidMount() {
    this.renderService.init(this.rootRef.current!);
  }

  public onFilter = () => {
    this.parserService.getGraphByFilter();
  };

  public render() {
    return (
      <div className="canvas-root">
        <div className="panels">
          <FilterPanel onSubmit={this.onFilter} />
        </div>
        <div id="canvas" ref={this.rootRef} />
      </div>
    );
  }
}
