import React, { Component, ReactNode } from "react";

interface IButtonProps {
  title: string;
  content: ReactNode;
  onClick: () => void;
}

export class Button extends Component<IButtonProps> {
  public render() {
    return (
      <button
        title={this.props.title}
        className="button"
        onClick={this.props.onClick}
      >
        {this.props.content}
      </button>
    );
  }
}
