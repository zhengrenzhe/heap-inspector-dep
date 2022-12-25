import React, { Component } from "react";
import { Card, Table, Text } from "@mantine/core";

import { __, inject } from "@web/common";
import { OmniService } from "@web/workbench/omnibox/omniService";

export class Meta extends Component {
  @inject()
  private omniService: OmniService;

  public override async componentDidMount() {
    const meta = await this.omniService.getMeta();
    this.setState(meta);
  }

  public override render() {
    const data = [
      {
        label: __("node_count"),
        value: this.state.node_count,
      },
      {
        label: __("edge_count"),
        value: this.state.edge_count,
      },
      {
        label: __("file_size"),
        value: `${Math.ceil(this.state.file_size / 1024 / 1024)} MB`,
      },
      {
        label: __("file_path"),
        value: this.state.file_path,
      },
    ];

    return (
      <Card shadow="md" radius="md" p="md" mt={20}>
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
      </Card>
    );
  }

  public override state = {
    node_count: 0,
    edge_count: 0,
    file_size: 0,
    file_path: "",
  };
}
