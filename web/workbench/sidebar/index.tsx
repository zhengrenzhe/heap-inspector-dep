import React, { Component, Fragment, ReactNode } from "react";
import { NavLink } from "react-router-dom";
import { Button, theme } from "antd";

import { cx, getContributions } from "@web/common";
import { IWorkbenchPageContribution } from "@web/workbench/contributions";

import "./style.less";

export class Sidebar extends Component {
  @getContributions(IWorkbenchPageContribution)
  private workbenchPageContributions: IWorkbenchPageContribution[];

  private get pages() {
    return this.workbenchPageContributions.sort((a, b) => a.order - b.order);
  }

  private get topButtons() {
    return this.pages.filter((p) => p.direction != "bottom");
  }

  private get bottomButtons() {
    return this.pages.filter((p) => p.direction === "bottom");
  }

  public override render() {
    return (
      <SideBarRoot>
        {this.renderButtons(this.topButtons)}
        <div className="spacer" />
        {this.renderButtons(this.bottomButtons)}
      </SideBarRoot>
    );
  }

  private renderButtons(items: IWorkbenchPageContribution[]) {
    return items.map((page) => (
      <Fragment key={page.id}>
        {page.path ? (
          <NavSidebarButton page={page} />
        ) : (
          <NormalSideBarButton page={page} />
        )}
      </Fragment>
    ));
  }
}

function NavSidebarButton(props: { page: IWorkbenchPageContribution }) {
  return (
    <NavLink to={props.page.path ?? ""}>
      {({ isActive }) => (
        <SideBarButton page={props.page} isActive={isActive} />
      )}
    </NavLink>
  );
}

function NormalSideBarButton(props: { page: IWorkbenchPageContribution }) {
  return <SideBarButton page={props.page} />;
}

function SideBarButton(props: {
  page: IWorkbenchPageContribution;
  isActive?: boolean;
}) {
  return (
    <Button
      type="text"
      icon={props.page.icon}
      className={cx({
        "sidebar-button": true,
        "sidebar-button-active": !!props.isActive,
      })}
      title={props.page.name}
    />
  );
}

function SideBarRoot(props: { children: ReactNode }) {
  const { token } = theme.useToken();
  return (
    <div
      className="sidebar"
      style={{ borderRight: `1px solid ${token.colorBorder}` }}
    >
      {props.children}
    </div>
  );
}
