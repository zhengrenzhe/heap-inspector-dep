import { SnapshotParser } from "@wasm";

import { i18n, I18n } from "@/i18n";

import { ViewModel } from "./ViewModel";
import { RenderService } from "./RenderService";
import { INodeDetailInfo } from "./type";

class _SnapshotService {
  public viewModel = new ViewModel();

  private parser: SnapshotParser | undefined;

  public init() {
    RenderService.init();
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
    };
  }

  public refreshGraph = () => {
    const result = this.parser?.get_graph_with_condition(this.viewModel.filter);
    RenderService.render(result);
  };

  public getNodeInfo(id: string) {
    return this.parser?.get_node_info_by_id(id) as INodeDetailInfo;
  }
}

export const SnapshotService = new _SnapshotService();
(window as any).SnapshotService = SnapshotService;
