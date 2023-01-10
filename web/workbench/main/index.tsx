import React, { Component } from "react";
import { Route, Routes } from "react-router-dom";

import { getContributions } from "@web/common";
import { IWorkbenchPageContribution } from "@web/workbench/contributions";

import "./style.less";

export class Main extends Component {
  @getContributions(IWorkbenchPageContribution)
  private workbenchPageContributions: IWorkbenchPageContribution[];

  private get routes() {
    return this.workbenchPageContributions
      .map((p) => ({
        id: p.id,
        path: p.path,
        exact: p.path === "/",
        view: p.view,
      }))
      .filter((p) => p.path);
  }

  public override render() {
    return (
      <div className="main">
        <Routes>
          {this.routes.map((r) => (
            <Route key={r.id} path={r.path!} element={r.view} />
          ))}
        </Routes>
      </div>
    );
  }
}
