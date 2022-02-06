import React, { Component } from "react";
import { observer } from "mobx-react";
import { CgSoftwareUpload } from "react-icons/cg";
import { IoRecordingOutline } from "react-icons/io5";

import { i18n } from "@/i18n";
import { LogService, ParserService } from "@/service";
import { inject } from "@/util";
import { Button, Text, Paper, Group, Badge } from "@mantine/core";

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
      this.logService.setMsg("load-done");
      this.parserService.fromBuffer(reader.result as ArrayBuffer);
    };
  };

  public render() {
    return (
      <Paper padding="sm" shadow="xs" radius={0} withBorder>
        <Group>
          <Text size="md" variant="gradient" weight={700}>
            Heap Visualization
          </Text>

          <Button
            compact
            onClick={this.onStartRecordClick}
            leftIcon={<IoRecordingOutline />}
            variant="white"
          >
            {i18n("record")}
          </Button>

          <Button
            compact
            onClick={this.onUploadClick}
            leftIcon={<CgSoftwareUpload />}
            variant="white"
          >
            {i18n("upload")}
          </Button>

          <Badge radius="sm" size="md" variant="dot" color="teal">
            {this.logService.viewModel.msg}
          </Badge>
        </Group>
      </Paper>
    );
  }
}
