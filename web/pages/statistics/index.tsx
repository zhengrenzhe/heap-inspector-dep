import React from "react";
import { VscPieChart } from "react-icons/vsc";

import { IWorkbenchPageContribution } from "@web/workbench/contributions";
import { __, contributionImplement } from "@web/common";
import { StatisticsView } from "@web/pages/statistics/view";

@contributionImplement()
export class Statistics extends IWorkbenchPageContribution {
  public id = "statistics";

  public icon = (<VscPieChart size={26} />);

  public order = 1;

  public override path = "/statistics";

  public override view = (<StatisticsView />);

  public override name = __("statistics");
}
