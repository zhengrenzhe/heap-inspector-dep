import { singleton } from "tsyringe";
import { Graph, Minimap, IG6GraphEvent } from "@antv/g6";
import { action, makeObservable, observable } from "mobx";

import { IEdgeDetailInfo, INodeDetailInfo, IResult } from "@wasm";
import { LogService, ParserService } from "@/service";
import { inject } from "@/util";
import { INodeInfoType } from "@/types";

export interface ITargets {
  id: number;
  type: "node" | "edge";
}

@singleton()
export class RenderService {
  private graph: Graph | undefined;

  @inject()
  private parserService!: ParserService;

  @inject()
  private logService!: LogService;

  public viewModel = new ViewModel();

  private minimap = new Minimap({
    size: [180, 120],
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
          "click-select",
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

    this.graph.on("nodeselectchange", this.onSelect);

    this.graph.on("mouseenter", this.onHover);
  }

  private async fetchData(targets: ITargets[]) {
    return Promise.all(
      targets.map((target) =>
        target.type === "node"
          ? this.parserService.getNodeInfo(target.id)
          : this.parserService.getEdgeInfo(target.id)
      )
    );
  }

  private onHover = async (e: IG6GraphEvent) => {
    if (e.item) {
      const type = e.item._cfg?.type as "edge" | "node";

      const targets = (() => {
        if (type === "edge") {
          if ("edge_index" in (e.item?._cfg?.model ?? ({} as any))) {
            return {
              id: parseInt((e.item?._cfg?.model as any).edge_index),
              type,
            };
          }
          return null;
        }
        if (type === "node") {
          if (e.item._cfg?.id) {
            return {
              id: parseInt(e.item._cfg.id),
              type,
            };
          }
          return null;
        }
      })();

      if (!targets) return;
      const data = await this.fetchData([targets]);
      this.viewModel.setInfo("hover", data);
    }
  };

  private onSelect = async (e: IG6GraphEvent) => {
    const ids: ITargets[] = (e.selectedItems as any).nodes.map((n: any) => ({
      id: parseInt(n._cfg.id),
      type: "node" as const,
    }));
    const data = await this.fetchData(ids);
    this.viewModel.setInfo("select", data);
  };

  public render(data: IResult) {
    this.logService.setMsg("rendering", [
      data.nodes.length.toString(),
      data.edges.length.toString(),
    ]);
    this.graph?.data(data);
    this.graph?.render();
  }
}

class ViewModel {
  constructor() {
    makeObservable(this);
  }

  @observable
  public infos = new Map<
    INodeInfoType,
    (INodeDetailInfo | IEdgeDetailInfo)[]
  >();

  @action
  public setInfo(
    type: INodeInfoType,
    nodes: (INodeDetailInfo | IEdgeDetailInfo)[]
  ) {
    this.infos.set(
      type,
      nodes.filter((n) => !!n)
    );
  }
}
