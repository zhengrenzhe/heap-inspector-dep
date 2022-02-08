import React, { Component } from "react";
import { observer } from "mobx-react";
import { Table, Text, Card, Tooltip, Badge, Box } from "@mantine/core";

import { RenderService } from "@/service";
import { inject } from "@/util";
import { INodeInfoType, NodeInfoTypeSort } from "@/types";
import { IEdgeDetailInfo, INodeDetailInfo } from "@wasm";
import { I18n, i18n } from "@/i18n";

@observer
export class InfoPanel extends Component {
  @inject()
  private renderService!: RenderService;

  private renderType(
    type: INodeInfoType,
    infos?: (INodeDetailInfo | IEdgeDetailInfo)[]
  ) {
    if (!infos || infos.length === 0) return null;

    return (
      <Card
        shadow="sm"
        radius="md"
        padding="md"
        style={{ marginBottom: 20, width: 320 }}
        key={type}
      >
        <Box style={{ marginBottom: 4 }}>
          <Text weight={500}>{i18n(type)}</Text>
        </Box>
        {infos.map((info, index) => (
          <Table
            striped
            highlightOnHover
            style={{ display: "flex", overflow: "hidden", marginBottom: 10 }}
            key={index}
          >
            <tbody style={{ overflow: "hidden", width: "100%" }}>
              {Object.entries(info).map(([key, value]) => (
                <tr key={key} style={{ width: "100%" }}>
                  <td>
                    <Badge
                      color="blue"
                      variant="outline"
                      style={{ flexShrink: 0 }}
                    >
                      {i18n(key as keyof I18n)}
                    </Badge>
                  </td>
                  <td style={{ width: "100%" }}>
                    <Tooltip
                      label={value}
                      transition="fade"
                      transitionDuration={200}
                      wrapLines
                      width={220}
                      delay={100}
                      withArrow
                      allowPointerEvents
                    >
                      <Text size="sm" lineClamp={2}>
                        {value}
                      </Text>
                    </Tooltip>
                  </td>
                </tr>
              ))}
            </tbody>
          </Table>
        ))}
      </Card>
    );
  }

  public render() {
    return (
      <>
        {NodeInfoTypeSort.map((type) =>
          this.renderType(type, this.renderService.viewModel.infos.get(type))
        )}
      </>
    );
  }
}
