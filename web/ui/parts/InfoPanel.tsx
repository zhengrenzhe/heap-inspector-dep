import React, { Component } from "react";
import { observer } from "mobx-react";

import { Panel } from "@/ui/atoms";
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
    console.log(type, infos);
    if (!infos || infos.length === 0) return null;
    return (
      <div className="type-area">
        <div className="type-name">{type}</div>
        {infos.map((info) => (
          <ul className="fields">
            {Object.entries(info).map(([key, value]) => (
              <li className="fields-row">
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
      <Panel name="node-info">
        {NodeInfoTypeSort.map((type) =>
          this.renderType(type, this.renderService.viewModel.infos.get(type))
        )}
      </Panel>
    );
  }
}
