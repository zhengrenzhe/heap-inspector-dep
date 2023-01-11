import React, { cloneElement, Component } from "react";
import { Dropdown } from "antd";
import { observer } from "mobx-react";
import { VscColorMode } from "react-icons/vsc";
import { MdDarkMode, MdLightMode } from "react-icons/md";

import { __, contributionImplement, inject } from "@web/common";
import { IWorkbenchPageContribution } from "@web/workbench/contributions";
import { ITheme, WorkbenchService } from "@web/service";

import "./style.less";

@contributionImplement()
export class ThemeToggle extends IWorkbenchPageContribution {
  public id = "theme-toggle";

  public icon = (<ToggleIcon />);

  public order = 999;

  public override direction = "bottom" as const;

  public override name = __("theme");
}

const items = [
  {
    key: "auto",
    label: __("os-default"),
    icon: <VscColorMode />,
  },
  {
    key: "dark",
    label: __("dark"),
    icon: <MdDarkMode />,
  },
  {
    key: "light",
    label: __("light"),
    icon: <MdLightMode />,
  },
];

@observer
class ToggleIcon extends Component {
  @inject()
  public wbService: WorkbenchService;

  public override render() {
    const theme = this.wbService.viewModel.theme;
    const curIcon = items.find((t) => t.key === theme)?.icon;
    return (
      <Dropdown
        menu={{
          items,
          selectable: true,
          selectedKeys: [theme],
          onSelect: (val) =>
            this.wbService.viewModel.setTheme(val.key as ITheme),
        }}
        trigger={["click"]}
      >
        {curIcon && cloneElement(curIcon, { size: 22 })}
      </Dropdown>
    );
  }
}
