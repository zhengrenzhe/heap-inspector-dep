import React, { useState } from "react";
import { createRoot } from "react-dom/client";
import "reflect-metadata";
import { ColorScheme, MantineProvider } from "@mantine/core";
import { configure } from "mobx";

import { Workbench } from "@web/page/workbench";

configure({
  useProxies: "always",
  enforceActions: "always",
});

const rootDom = document.createElement("div");
rootDom.id = "app-root";
document.body.append(rootDom);

function App() {
  const [colorScheme, setColorScheme] = useState<ColorScheme>("light");
  const toggleColorScheme = () =>
    setColorScheme(colorScheme === "dark" ? "light" : "dark");

  return (
    <MantineProvider theme={{ colorScheme }}>
      <Workbench cs={colorScheme} toggleColorScheme={toggleColorScheme} />
    </MantineProvider>
  );
}

const root = createRoot(rootDom);
root.render(<App />);
