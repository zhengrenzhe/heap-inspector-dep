import React, { Component } from "react";
import { render } from "react-dom";

import { Canvas } from "./canvas";
import { Header } from "./header";

import "./style.less";

class UI extends Component {
  public render() {
    return (
      <div className="app-root">
        <Header />
        <Canvas />
      </div>
    );
  }
}

export function mountUI() {
  render(<UI />, document.getElementById("root"));
}
