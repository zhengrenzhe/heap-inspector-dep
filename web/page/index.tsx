import React from "react";
import { createRoot } from "react-dom/client";
import "reflect-metadata";
import { MantineProvider } from "@mantine/core";
import { configure } from "mobx";

import { Workbench } from "@web/page/workbench";

configure({
  useProxies: "always",
  enforceActions: "always",
});

const rootDom = document.createElement("div");
rootDom.id = "app-root";
document.body.append(rootDom);

const App = (
  <MantineProvider theme={{ colorScheme: "light" }}>
    <Workbench />
  </MantineProvider>
);

const root = createRoot(rootDom);
root.render(App);
