import React, { Component } from "react";
import { Loader, Text } from "@mantine/core";

import { __ } from "@web/common";

import "./progress.less";

export class Progress extends Component {
  public override render() {
    return (
      <div className="progress">
        <Loader color="teal" size="xl" />
        <span className="label">
          <Text c="teal" fz="md">
            {__("loading")}
          </Text>
        </span>
      </div>
    );
  }
}
