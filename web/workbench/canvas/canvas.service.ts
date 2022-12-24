import { Graph, GraphData, Minimap } from "@antv/g6";

import { injectable } from "@web/common";

@injectable()
export class CanvasService {
  public graph: Graph;

  private minimap = new Minimap({
    size: [200, 160],
    type: "keyShape",
  });

  public init() {
    this.graph = new Graph({
      container: "canvas",
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
          "click-select",
        ],
      },
      plugins: [this.minimap],
      animate: true,
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

  public render(data: GraphData) {
    this.graph.data(data);
    this.graph.render();
  }
}
