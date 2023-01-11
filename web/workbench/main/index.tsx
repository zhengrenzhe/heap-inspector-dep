import React from "react";
import { Route, Routes } from "react-router-dom";

import { useContributions } from "@web/common";
import { IWorkbenchPageContribution as IPage } from "@web/workbench/contributions";

import "./style.less";

export function Main() {
  const routes = useContributions(IPage)
    .filter((p) => p.path)
    .map((p) =>
      Object.assign(p, { path: p.path ?? "", exact: p.path === "/" })
    );

  return (
    <div className="main">
      <Routes>
        {routes.map((r) => (
          <Route key={r.id} path={r.path} element={r.view} />
        ))}
      </Routes>
    </div>
  );
}
