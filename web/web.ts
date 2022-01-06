console.log("web is working...");

import init, { add } from "../pkg";

(async () => {
  await init();
  console.log(add(1, 3));
})();
