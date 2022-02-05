import { IFilterCondition } from "./filter";
import { IResult } from "@wasm";
import { I18n } from "@/i18n";

export enum WorkerEventName {
  Inited,
  Log,
  GetGraph,
  ReturnGraph,
  GetNode,
}

export abstract class BaseWorkerEvent {
  public abstract readonly name: WorkerEventName;
}

export class WorkerInitedEvent extends BaseWorkerEvent {
  public readonly name = WorkerEventName.Inited;
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

export class WorkerGetGraphEvent extends BaseWorkerEvent {
  public readonly name = WorkerEventName.GetGraph;

  constructor(public readonly cond: IFilterCondition) {
    super();
  }
}

export class WorkerReturnGraphEvent extends BaseWorkerEvent {
  public readonly name = WorkerEventName.ReturnGraph;

  constructor(public readonly graph: IResult) {
    super();
  }
}

export class WorkerGetNodeEvent extends BaseWorkerEvent {
  public readonly name = WorkerEventName.GetNode;

  constructor(public readonly id: string) {
    super();
  }
}
