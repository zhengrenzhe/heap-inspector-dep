import React, { Component, Fragment } from "react";
import { NavLink } from "react-router-dom";

import { getContributions } from "@web/common";
import { IWorkbenchPageContribution } from "@web/workbench/contributions";
import { Icon } from "@web/atoms";

import "./style.less";

export class Sidebar extends Component {
  @getContributions(IWorkbenchPageContribution)
  private workbenchPageContributions: IWorkbenchPageContribution[];

  private get pages() {
    return this.workbenchPageContributions.sort((a, b) => a.order - b.order);
  }

  public override render() {
    return (
      <div className="sidebar">
        {this.pages.map((page) => (
          <Fragment key={page.id}>
            <NavLink to={page.path} className="sidebar-link" title={page.name}>
              {({ isActive }) => (
                <SideBarButton name={page.icon} isActive={isActive} />
              )}
            </NavLink>
          </Fragment>
        ))}
      </div>
    );
  }
}

function SideBarButton(props: { name: string; isActive: boolean }) {
  return (
    <div
      className={`sidebar-icon ${props.isActive ? "sidebar-icon-active" : ""}`}
    >
      <Icon name={props.name} />
    </div>
  );
}
