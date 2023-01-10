import React from "react";
import { createRoot } from "react-dom/client";
import "reflect-metadata";
import { configure } from "mobx";

import "@web/pages";
import { Workbench } from "@web/workbench";

configure({
  useProxies: "always",
  enforceActions: "always",
});

const rootDom = document.createElement("div");
rootDom.id = "app-root";
document.body.append(rootDom);

function App() {
  return <Workbench />;
}

const root = createRoot(rootDom);
root.render(<App />);
