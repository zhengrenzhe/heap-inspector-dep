import { SnapshotParser } from "@wasm";

class Srv {
  private parser: SnapshotParser | undefined;

  public async parseLocal() {
    const [fileHandler] = await showOpenFilePicker();
    const reader = new FileReader();

    reader.onload = () => {
      const buffer = new Uint8Array(reader.result as ArrayBuffer);
      this.parser = new SnapshotParser(buffer.byteLength);
      this.parser.load(buffer);
    };

    const file = await fileHandler.getFile();
    reader.readAsArrayBuffer(file);
  }
}

export const Service = new Srv();
