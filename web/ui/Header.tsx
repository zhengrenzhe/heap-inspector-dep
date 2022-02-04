import React, { Component } from "react";
import { observer } from "mobx-react";
import { CgSoftwareUpload } from "react-icons/cg";
import { IoRecordingOutline } from "react-icons/io5";

import { i18n } from "@/i18n";
import { LogService, ParserService } from "@/service";
import { inject } from "@/util";
import { Button } from "@/ui/atoms";

@observer
export class Header extends Component {
  @inject()
  private logService!: LogService;

  @inject()
  private parserService!: ParserService;

  private onStartRecordClick = () => {};

  private onUploadClick = async () => {
    const [fileHandler] = await showOpenFilePicker();
    const reader = new FileReader();

    this.logService.setMsg("loading");
    const file = await fileHandler.getFile();
    reader.readAsArrayBuffer(file);

    reader.onload = () => {
      const buffer = new Uint8Array(reader.result as ArrayBuffer);
      this.logService.setMsg("load-done");
      this.parserService.fromBuffer(buffer);
    };
  };

  public render() {
    return (
      <div className="header">
        <span className="logo">Heap Visualization</span>

        <Button
          title={i18n("start-record")}
          content={<IoRecordingOutline />}
          onClick={this.onStartRecordClick}
        />

        <Button
          title={i18n("upload")}
          content={<CgSoftwareUpload />}
          onClick={this.onUploadClick}
        />

        <div className="msg">{this.logService.viewModel.msg}</div>
      </div>
    );
  }
}
