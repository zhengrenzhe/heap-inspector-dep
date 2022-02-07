import React, { Component } from "react";
import { observer } from "mobx-react";
import { Paper } from "@mantine/core";

import { RenderService } from "@/service";
import { inject } from "@/util";
import { INodeInfoType, NodeInfoTypeSort } from "@/types";
import { INodeDetailInfo } from "@wasm";
import { I18n, i18n } from "@/i18n";

@observer
export class InfoPanel extends Component {
  @inject()
  private renderService!: RenderService;

  private renderType(type: INodeInfoType, infos?: INodeDetailInfo[]) {
    if (!infos || infos.length === 0) return null;
    return (
      <div className="type-area" key={type}>
        <div className="type-name">{type}</div>
        {infos.map((info) => (
          <ul className="fields" key={info.id}>
            {Object.entries(info).map(([key, value]) => (
              <li className="fields-row" key={key}>
                <span className="field-key">{i18n(key as keyof I18n)}</span>
                <span className="field-value">{value}</span>
              </li>
            ))}
          </ul>
        ))}
      </div>
    );
  }

  public render() {
    return (
      <Paper padding="md" shadow="sm" radius="md">
        {NodeInfoTypeSort.map((type) =>
          this.renderType(type, this.renderService.viewModel.infos.get(type))
        )}
      </Paper>
    );
  }
}
