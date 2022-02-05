import React, { Component } from "react";

import { i18n, I18n } from "@/i18n";

interface IPanelProps {
  name: keyof I18n;
}

export class Panel extends Component<IPanelProps> {
  public render() {
    return (
      <div className={`panel ${this.props.name}`}>
        <span className="panel-name">{i18n(this.props.name)}</span>
        {this.props.children}
      </div>
    );
  }
}
