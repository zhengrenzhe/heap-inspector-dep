import React, { useEffect, useState } from "react";
import { Progress as ProgressBar } from "@mantine/core";
import { useInterval } from "@mantine/hooks";

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
    </div>
  );
}
