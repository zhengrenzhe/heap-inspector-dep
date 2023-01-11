import React, { Fragment } from "react";
import { NavLink } from "react-router-dom";
import { Button } from "antd";

import { cx, useColor, useContributions } from "@web/common";
import { IWorkbenchPageContribution as IPage } from "@web/workbench/contributions";

import "./style.less";

export function Sidebar() {
  const pages = useContributions(IPage).sort((a, b) => a.order - b.order);

  const topButtons = pages.filter((p) => p.direction != "bottom");
  const bottomButtons = pages.filter((p) => p.direction === "bottom");

  const { colorBorder } = useColor();

  return (
    <div
      className="sidebar"
      style={{ borderRight: `1px solid ${colorBorder}` }}
    >
      <RenderButtons items={topButtons} />
      <div className="spacer" />
      <RenderButtons items={bottomButtons} />
    </div>
  );
}

function RenderButtons(props: { items: IPage[] }) {
  return (
    <>
      {props.items.map((page) => (
        <Fragment key={page.id}>
          {page.path ? (
            <NavLink to={page.path}>
              {(arg) => <SideBarButton page={page} isActive={arg.isActive} />}
            </NavLink>
          ) : (
            <SideBarButton page={page} />
          )}
        </Fragment>
      ))}
    </>
  );
}

function SideBarButton(props: { page: IPage; isActive?: boolean }) {
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
