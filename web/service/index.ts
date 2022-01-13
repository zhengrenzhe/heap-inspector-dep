import { Graph } from "@antv/g6";
import { SnapshotParser } from "@wasm";

import { i18n, I18n } from "@/i18n";

import { ViewModel } from "./model";
import { ISearchResult } from "./type";

class Srv {
  public viewModel = new ViewModel();

  private parser: SnapshotParser | undefined;

  private graph: Graph | undefined;

  public initGraph() {
    const canvas = document.getElementById("canvas")!;
    const rect = canvas.getBoundingClientRect();
    this.graph = new Graph({
      container: "canvas",
      width: rect.width,
      height: rect.height,
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
    this.graph?.data(data as any);
  }

  public set_msg(msg: keyof I18n) {
    this.viewModel.setMsg(i18n(msg));
  }

  public set_msg2(m1: keyof I18n, m2: string) {
    this.viewModel.setMsg(`${i18n(m1)} ${m2}`);
  }

  public async parseLocal() {
    const [fileHandler] = await showOpenFilePicker();
    const reader = new FileReader();

    this.set_msg("loading");
    const file = await fileHandler.getFile();
    reader.readAsArrayBuffer(file);

    reader.onload = () => {
      const buffer = new Uint8Array(reader.result as ArrayBuffer);
      this.set_msg("load-done");
      this.parser = new SnapshotParser(buffer);
      const result = this.parser.get_graph() as ISearchResult;
      this.render(result);
    };
  }
}

export const Service = new Srv();
(window as any).Service = Service;
