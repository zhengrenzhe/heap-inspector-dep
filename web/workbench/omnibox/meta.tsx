import React, { Component } from "react";
import { Table, Text } from "@mantine/core";
import { observer } from "mobx-react";
import { Pie } from "@ant-design/plots";

import { __, inject } from "@web/common";
import { OmniService } from "@web/workbench/omnibox/omniService";

@observer
export class Meta extends Component {
  @inject()
  private omniService: OmniService;

  private get meta() {
    return this.omniService.viewModel.meta;
  }

  private get statistics() {
    return this.omniService.viewModel.statistics;
  }

  public override render() {
    return (
      <>
        {this.renderMeta()}
        {this.renderStatistics()}
      </>
    );
  }

  private renderMeta() {
    if (!this.meta) return null;

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

  private renderStatistics() {
    if (!this.statistics) return null;

    const data = Object.entries(this.statistics.percent)
      .map(([type, value]) => ({ type, value: Math.floor(value / 1024) }))
      .sort((a, b) => a.type.length - b.type.length);

    const total_bytes = `${Math.floor(this.statistics.total_bytes / 1024)} KB`;

    return (
      <div>
        <Pie
          data={data}
          angleField="value"
          colorField="type"
          height={500}
          width={290}
          appendPadding={[0, 10, 0, 10]}
          autoFit={false}
          innerRadius={0.6}
          color={[
            "#D9480F",
            "#E67700",
            "#FFA94D",
            "#5C940D",
            "#2B8A3E",
            "#087F5B",
            "#0B7285",
            "#1864AB",
            "#364FC7",
            "#5F3DC4",
            "#862E9C",
            "#A61E4D",
            "#F06595",
            "#C92A2A",
            "#495057",
          ]}
          label={false}
          statistic={{
            title: false,
            content: {
              style: {
                whiteSpace: "pre-wrap",
                overflow: "hidden",
                textOverflow: "ellipsis",
                fontSize: "18px",
                lineHeight: 1.4,
              },
              formatter: (datum) => {
                if (!datum) {
                  return `${__("total")}\n${total_bytes}`;
                }
                return `${datum["type"]}\n${datum["value"]} KB`;
              },
            },
          }}
          interactions={[
            {
              type: "element-selected",
            },
            {
              type: "element-active",
            },
            {
              type: "pie-statistic-active",
            },
          ]}
          state={{
            active: {
              style: {
                lineWidth: 3,
                stroke: "#101113",
              },
            },
          }}
          legend={{
            layout: "horizontal",
            position: "bottom",
            height: 999,
            autoFit: false,
            flipPage: false,
            itemSpacing: 4,
            maxRow: 10,
            itemName: {
              style: {
                fontSize: 13,
                lineHeight: 13,
              },
            },
            itemHeight: 13,
            marker: {
              symbol: "square",
            },
          }}
        />
      </div>
    );
  }
}
