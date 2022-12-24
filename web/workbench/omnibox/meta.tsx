import React, { Component } from "react";
import { Badge, List, Text } from "@mantine/core";

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
    return (
      <List spacing="xs" size="sm" listStyleType="none">
        <List.Item>
          <Text fz="sm" c="teal">
            <Badge color="teal" radius="sm" variant="outline" mr={6}>
              {__("node_count")}
            </Badge>
            {this.state.node_count}
          </Text>
        </List.Item>
        <List.Item>
          <Text fz="sm" c="teal">
            <Badge color="teal" radius="sm" variant="outline" mr={6}>
              {__("edge_count")}
            </Badge>
            {this.state.edge_count}
          </Text>
        </List.Item>
        <List.Item>
          <Text fz="sm" c="teal">
            <Badge color="teal" radius="sm" variant="outline" mr={6}>
              {__("file_size")}
            </Badge>
            {Math.ceil(this.state.file_size / 1024 / 1024)} MB
          </Text>
        </List.Item>
      </List>
    );
  }

  public override state = {
    node_count: 0,
    edge_count: 0,
    file_size: 0,
  };
}
