console.log("web is working...");

import init, { SnapshotParser } from "../pkg";

(async () => {
  await init();

  document.getElementById("load")?.addEventListener("click", async () => {
    const [file] = await showOpenFilePicker();
    const f = await file.getFile();
    const reader = new FileReader();
    reader.onload = () => {
      const buffer = reader.result as ArrayBuffer;

      const parser = new SnapshotParser(buffer.byteLength);
      parser.load(new Uint8Array(buffer));
    };
    reader.readAsArrayBuffer(f);
  });
})();
