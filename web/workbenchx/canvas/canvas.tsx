import React, { Component } from "react";

import { CanvasService } from "@web/workbenchx/canvas/canvas.service";
import { inject } from "@web/common";

import "./canvas.less";

export class Canvas extends Component {
  @inject()
  private canvasService: CanvasService;

  public override componentDidMount() {
    this.canvasService.init();
  }

  public override render() {
    return <div id="canvas"></div>;
  }
}
