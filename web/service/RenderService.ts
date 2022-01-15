import { Graph, Tooltip, Minimap } from "@antv/g6";

import { SnapshotService } from "@/service/SnapshotService";
import { I18n, i18n } from "@/i18n";

import { ISearchResult } from "./type";

const minimap = new Minimap({
  size: [150, 100],
  type: "keyShape",
});

const tooltip = new Tooltip({
  offsetX: 0,
  offsetY: 0,
  itemTypes: ["node", "edge"],
  trigger: "click",
  getContent(e) {
    const id: string = e!.item!.get("id");
    const info = Object.entries(SnapshotService.getNodeInfo(id)).map(
      ([key, value]) =>
        `<div class="node-info-row">
          <span>${i18n(key as keyof I18n)}:</span>
          <span>${value}</span>
        </div>`
    );
    return `<div class="node-info-panel">${info.join("")}</div>`;
  },
});

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
        default: [
          {
            type: "zoom-canvas",
            enableOptimize: true,
            optimizeZoom: 0.9,
          },
          {
            type: "drag-canvas",
            enableOptimize: true,
          },
          "drag-node",
        ],
      },
      plugins: [tooltip, minimap],
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
  }

  public render(data: ISearchResult) {
    console.log(data);
    this.graph?.data(data);
    this.graph?.render();
  }
}

export const RenderService = new _RenderService();
