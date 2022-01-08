import init from "@wasm";
import { mountUI } from "@/ui";

(async () => {
  await init();
  mountUI();
})();
