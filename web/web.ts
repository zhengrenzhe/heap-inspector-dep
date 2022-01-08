import init from "../pkg";
import { mountUI } from "@/ui";

(async () => {
  await init();
  mountUI();
})();
