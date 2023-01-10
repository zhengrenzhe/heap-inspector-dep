import { ReactNode } from "react";

import { contribution } from "@web/common";

@contribution()
export abstract class IWorkbenchPageContribution {
  public abstract id: string;

  public abstract icon: ReactNode;

  public abstract order: number;

  public direction?: "top" | "bottom";

  public path?: string;

  public view?: ReactNode;

  public name?: string;
}
