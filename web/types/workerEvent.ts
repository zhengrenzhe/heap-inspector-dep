export enum WorkerEventName {
  Inited,
  Log,
}

export abstract class BaseWorkerEvent {
  public abstract readonly name: WorkerEventName;
}

export class WorkerInitedEvent extends BaseWorkerEvent {
  public readonly name = WorkerEventName.Inited;
}

export class WorkerLogEvent extends BaseWorkerEvent {
  public readonly name = WorkerEventName.Log;

  constructor(public readonly message: [string] | [string, string]) {
    super();
  }
}
