import React, { Component } from "react";

import { Omnibox } from "@web/page/canvas/omnibox";

import "./style.less";

export class Canvas extends Component {
  public override render() {
    return (
      <div className="canvas">
        <Omnibox />
      </div>
    );
  }
}
