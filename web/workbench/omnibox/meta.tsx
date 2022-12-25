import React, { Component } from "react";
import { Table, Text } from "@mantine/core";
import { observer } from "mobx-react";

import { __, inject } from "@web/common";
import { OmniService } from "@web/workbench/omnibox/omniService";

@observer
export class Meta extends Component {
  @inject()
  private omniService: OmniService;

  private get meta() {
    return this.omniService.viewModel.meta;
  }

  public override render() {
    if (!this.meta) return;

    const data = [
      {
        label: __("node_count"),
        value: this.meta.node_count,
      },
      {
        label: __("edge_count"),
        value: this.meta.edge_count,
      },
      {
        label: __("file_size"),
        value: `${Math.ceil(this.meta.file_size / 1024 / 1024)} MB`,
      },
      {
        label: __("file_path"),
        value: this.meta.file_path,
      },
    ];

    return (
      <Table fontSize="xs" style={{ tableLayout: "fixed" }}>
        <tbody>
          {data.map((d, index) => (
            <tr key={index}>
              <td width={60}>
                <Text>{d.label}</Text>
              </td>
              <td>
                <Text
                  style={{
                    whiteSpace: "nowrap",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                  }}
                  title={d.value.toString()}
                >
                  {d.value}
                </Text>
              </td>
            </tr>
          ))}
        </tbody>
      </Table>
    );
  }
}
