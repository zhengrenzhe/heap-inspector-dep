import { singleton } from "tsyringe";
import { Graph, Minimap } from "@antv/g6";
import { IResult } from "@wasm";

@singleton()
export class RenderService {
  private graph: Graph | undefined;

  private minimap = new Minimap({
    size: [150, 100],
    type: "keyShape",
  });

  public init(ele: HTMLDivElement) {
    const rect = ele.getBoundingClientRect();
    this.graph = new Graph({
      container: ele,
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
      plugins: [this.minimap],
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

  public render(data: IResult) {
    this.graph?.data(data);
    this.graph?.render();
  }
}
