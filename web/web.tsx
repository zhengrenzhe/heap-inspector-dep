import React from "react";
import { render } from "react-dom";
import { configure } from "mobx";
import "reflect-metadata";

import { UI } from "./ui";

configure({
  useProxies: "always",
  enforceActions: "always",
});

render(<UI />, document.getElementById("root"));
