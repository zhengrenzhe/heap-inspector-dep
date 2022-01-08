import { SnapshotParser } from "@wasm";

import { ViewModel } from "./model";
import { i18n, I18n } from "@/i18n";

class Srv {
  public viewModel = new ViewModel();

  private parser: SnapshotParser | undefined;

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
      this.parser = new SnapshotParser(buffer.byteLength);
      this.parser.load(buffer);
    };
  }
}

export const Service = new Srv();
(window as any).Service = Service;
