import { ReactNode } from "react";

import { contribution } from "@web/common";

@contribution()
export abstract class IWorkbenchPageContribution {
  public abstract id: string;

  public abstract icon: string;

  public abstract order: number;

  public abstract path: string;

  public view?: ReactNode;

  public name?: string;
}
