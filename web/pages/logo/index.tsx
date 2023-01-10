import React from "react";

import { contributionImplement } from "@web/common";
import { IWorkbenchPageContribution } from "@web/workbench/contributions";

@contributionImplement()
export class Logo extends IWorkbenchPageContribution {
  public id = "icon";

  public icon = (<span>H</span>);

  public order = 0;

  public override path = "/";
}
