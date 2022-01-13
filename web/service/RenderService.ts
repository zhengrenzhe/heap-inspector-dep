import { Graph, Tooltip } from "@antv/g6";

const tooltip = new Tooltip({
  offsetX: 0,
  offsetY: 0,
  itemTypes: ["node", "edge"],
  trigger: "click",
  getContent(e) {
    console.log(e);
    return "<span>XX</span>";
  },
});

import { ISearchResult } from "./type";

class _RenderService {
  private graph: Graph | undefined;

  public init() {
    const canvas = document.getElementById("canvas")!;
    const rect = canvas.getBoundingClientRect();
    this.graph = new Graph({
      container: canvas,
      width: rect.width,
      height: rect.height,
      modes: {
        default: ["drag-canvas", "drag-node"],
      },
      plugins: [tooltip],
      defaultNode: {
        size: 10,
        style: {
          lineWidth: 2,
          stroke: "#5B8FF9",
          fill: "#C6E5FF",
        },
      },
      defaultEdge: {
        size: 1,
        color: "#e2e2e2",
        style: {
          endArrow: {
            path: "M 0,0 L 8,4 L 8,-4 Z",
            fill: "#e2e2e2",
          },
        },
      },
      layout: {
        type: "fruchterman",
        gpuEnabled: true,
        workerEnabled: true,
      },
    });

    this.attachEvent();
  }

  private attachEvent() {
    this.graph?.on("node:click", (e) => {
      const nodeId = parseInt(e.item!.get("id"));
      console.log(nodeId);
    });
    this.graph?.on("edge:click", (e) => {
      console.log(e);
    });
    this.graph?.on("node:mouseenter", (e) => {
      this.graph?.setItemState(e.item!, "active", true);
    });
    this.graph?.on("node:mouseleave", (e) => {
      this.graph?.setItemState(e.item!, "active", false);
    });
    this.graph?.on("edge:mouseenter", (e) => {
      this.graph?.setItemState(e.item!, "active", true);
    });
    this.graph?.on("edge:mouseleave", (e) => {
      this.graph?.setItemState(e.item!, "active", false);
    });
  }

  public render(data: ISearchResult) {
    console.log(data);
    this.graph?.data(data);
    this.graph?.render();
  }
}

export const RenderService = new _RenderService();
