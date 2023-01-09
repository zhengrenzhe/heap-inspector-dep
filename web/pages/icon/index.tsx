import { contributionImplement } from "@web/common";
import { IWorkbenchPageContribution } from "@web/workbench/contributions";

import "./icon.less";

@contributionImplement()
export class Icon extends IWorkbenchPageContribution {
  public id = "icon";

  public icon = "hubot";

  public order = 0;

  public override path = "/";
}
