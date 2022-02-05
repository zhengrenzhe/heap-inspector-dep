import { I18n } from "@/i18n";

export enum WorkerEventName {
  Log,
}

export abstract class BaseWorkerEvent {
  public abstract readonly name: WorkerEventName;
}

export class WorkerLogEvent extends BaseWorkerEvent {
  public readonly name = WorkerEventName.Log;

  constructor(
    public readonly message: keyof I18n,
    public readonly params?: string[]
  ) {
    super();
  }
}
