import React, { Component, ReactNode } from "react";
import { observer } from "mobx-react";
import { CgSoftwareUpload } from "react-icons/cg";
import { IoRecordingOutline } from "react-icons/io5";

import { i18n } from "@/i18n";
import { Service } from "@/service";

function Logo() {
  return <span className="logo">Heap Visualization</span>;
}

interface IButtonProps {
  title: string;
  content: ReactNode;
  onClick: () => void;
}

class Button extends Component<IButtonProps> {
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

@observer
export class Header extends Component {
  private parseLocal = () => {
    void Service.parseLocal();
  };

  private startRecord = () => {};

  public render() {
    return (
      <div className="header">
        <Logo />

        <Button
          title={i18n("start-record")}
          content={<IoRecordingOutline />}
          onClick={this.startRecord}
        />

        <Button
          title={i18n("upload")}
          content={<CgSoftwareUpload />}
          onClick={this.parseLocal}
        />

        <div className="msg">{Service.viewModel.msg}</div>
      </div>
    );
  }
}
