import { singleton } from "tsyringe";
import { Graph, Minimap, IG6GraphEvent } from "@antv/g6";
import { action, makeObservable, observable } from "mobx";

import { INodeDetailInfo, IResult } from "@wasm";
import { LogService, ParserService } from "@/service";
import { inject } from "@/util";
import { INodeInfoType } from "@/types";

@singleton()
export class RenderService {
  private graph: Graph | undefined;

  @inject()
  private parserService!: ParserService;

  @inject()
  private logService!: LogService;

  public viewModel = new ViewModel();

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

  private async fetchData(ids: number[]) {
    return Promise.all(ids.map((id) => this.parserService.getNodeInfo(id)));
  }

  private onHover = async (e: IG6GraphEvent) => {
    if (e.item) {
      const data = await this.fetchData([parseInt(e.item._cfg?.id!)]);
      this.viewModel.setInfo("hover", data);
    }
  };

  private onSelect = async (e: IG6GraphEvent) => {
    const ids: number[] = (e.selectedItems as any).nodes.map((n: any) =>
      parseInt(n._cfg.id)
    );
    const data = await this.fetchData(ids);
    this.viewModel.setInfo("select", data);
  };

  public render(data: IResult) {
    this.logService.setMsg("rendering");
    this.graph?.data(data);
    this.graph?.render();
    this.logService.setMsg("rendering-done");
  }
}

class ViewModel {
  constructor() {
    makeObservable(this);
  }

  @observable
  public infos = new Map<INodeInfoType, INodeDetailInfo[]>();

  @action
  public setInfo(type: INodeInfoType, nodes: INodeDetailInfo[]) {
    this.infos.set(type, nodes);
  }
}
