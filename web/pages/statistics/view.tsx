import React, { useEffect, useRef, useState } from "react";
import { Chart, ChartData } from "chart.js/auto";
import { Divider, List, Typography } from "antd";

import { APIService } from "@web/service";
import { __, useColor, useService } from "@web/common";

import "./style.less";

export function StatisticsView() {
  const ref = useRef<HTMLCanvasElement | null>(null);
  const chart = useRef<Chart | null>(null);
  const color = useColor();
  const [tableData, setTableData] = useState<
    {
      label: string;
      value: number | string;
    }[]
  >();

  const apiService = useService(APIService);

  useEffect(() => {
    apiService.getStatistics().then((res) => {
      if (!ref.current) return;

      const labels = Object.keys(res.percent);
      const data: ChartData = {
        labels,
        datasets: [
          {
            data: labels.map((k) => res.percent[k]) as number[],
            label: "bytes ",
            backgroundColor: [
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
            ],
          },
        ],
      };

      chart.current = new Chart(ref.current, {
        type: "doughnut",
        data,
        options: {
          animation: false,
          responsive: false,
          borderColor: color.colorText,
          plugins: {
            legend: {
              position: "right",
              align: "start",
            },
            title: {
              display: false,
            },
          },
        },
      });
    });

    apiService.getMeta().then((res) => {
      setTableData([
        {
          label: __("node_count"),
          value: res.node_count,
        },
        {
          label: __("edge_count"),
          value: res.edge_count,
        },
        {
          label: __("file_size"),
          value: `${Math.ceil(res.file_size / 1024 / 1024)} MB`,
        },
        {
          label: __("file_path"),
          value: res.file_path,
        },
      ]);
    });
  }, []);

  return (
    <div className="statistics">
      <div className="statistics-graph">
        <canvas ref={ref} style={{ width: 570, height: 300 }} />
      </div>
      <Divider />
      {tableData ? (
        <List
          size="small"
          // bordered
          split={false}
          dataSource={tableData}
          renderItem={(item) => (
            <List.Item>
              <Typography.Text>
                {item.label} {item.value}
              </Typography.Text>
            </List.Item>
          )}
        />
      ) : null}
    </div>
  );
}
