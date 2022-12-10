import React, { Component } from "react";
import { Paper, Text } from "@mantine/core";

export class OmniPanel extends Component {
  public override render() {
    return (
      <Paper shadow="md" radius="lg" p="md" withBorder id="omni-panel">
        <Text>
          Use it to create cards, dropdowns, modals and other components that
          require background with shadow.
        </Text>
      </Paper>
    );
  }
}
