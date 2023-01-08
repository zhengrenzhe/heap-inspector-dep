import React, { useState } from "react";
import { createRoot } from "react-dom/client";
import "reflect-metadata";
import { ColorScheme, MantineProvider } from "@mantine/core";
import { configure } from "mobx";

import { Workbench } from "@web/workbenchx/workbench";

configure({
  useProxies: "always",
  enforceActions: "always",
});

const rootDom = document.createElement("div");
rootDom.id = "app-root";
document.body.append(rootDom);

function App() {
  const [colorScheme, setColorScheme] = useState<ColorScheme>(
    (localStorage.getItem("colorScheme") as ColorScheme) ?? "dark"
  );
  const toggleColorScheme = () => {
    const newVal = colorScheme === "dark" ? "light" : "dark";
    localStorage.setItem("colorScheme", newVal);
    setColorScheme(newVal);
  };

  return (
    <MantineProvider theme={{ colorScheme, primaryColor: "teal" }}>
      <Workbench cs={colorScheme} toggleColorScheme={toggleColorScheme} />
    </MantineProvider>
  );
}

const root = createRoot(rootDom);
root.render(<App />);
