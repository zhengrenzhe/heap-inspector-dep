import React, { useEffect, useState } from "react";
import { Progress as ProgressBar, Text } from "@mantine/core";
import { useInterval } from "@mantine/hooks";

import { __ } from "@web/common";

import "./progress.less";

export function Progress() {
  const [count, setCount] = useState(0);
  const interval = useInterval(() => setCount((s) => s + 1), 1000);

  useEffect(() => {
    interval.start();
    return interval.stop;
  }, []);

  return (
    <div className="progress">
      <ProgressBar
        color="teal"
        size="sm"
        value={count}
        className="progress-bar"
      />
      <span className="label">
        <Text c="teal" fz="xs">
          {__("loading")}
        </Text>
      </span>
    </div>
  );
}
