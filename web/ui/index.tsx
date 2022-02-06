import React, { Component } from "react";

import { Canvas } from "./Canvas";
import { Header } from "./Header";

import "./style.less";

export class UI extends Component {
  public render() {
    return (
      <>
        <Header />
        <Canvas />
      </>
    );
  }
}
