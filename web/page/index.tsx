import React from "react";
import { createRoot } from "react-dom/client";
import "reflect-metadata";
import { MantineProvider } from "@mantine/core";
import { Workbench } from "@web/page/workbench";

import "./style.less";

const rootDom = document.createElement("div");
rootDom.id = "app-root";
document.body.append(rootDom);

const App = (
  <MantineProvider theme={{ colorScheme: "dark" }}>
    <Workbench />
  </MantineProvider>
);

const root = createRoot(rootDom);
root.render(App);
