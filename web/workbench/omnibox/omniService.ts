import axios from "axios";
import { action, makeObservable, observable } from "mobx";

import { API, inject, injectable } from "@web/common";
import { CanvasService } from "@web/workbench/canvas/canvas.service";

export interface IFilter {
  filter_from: string[];
  filter_name: string;
  self_size_mode: string;
  self_size: number;
  retained_size_mode: string;
  retained_size: number;
  depth_mode: string;
  depth: number;
  node_types: string[];
}

export interface IMeta {
  edge_count: number;
  node_count: number;
  file_size: number;
  file_path: string;
  node_types: string[];
  edge_types: string[];
}

export interface IStatistics {
  percent: Record<string, number>;
  total_bytes: number;
}

class ViewModel {
  @observable
  public filter: IFilter = {
    filter_from: ["constructor_name"],
    filter_name: "",
    self_size_mode: "more_than",
    self_size: 0,
    retained_size_mode: "more_than",
    retained_size: 0,
    depth_mode: "more_than",
    depth: 0,
    node_types: [],
  };

  @observable
  public searching = false;

  @observable
  public meta: IMeta | null = null;

  @observable
  public statistics: IStatistics | null = null;

  constructor() {
    makeObservable(this);
  }

  @action
  public setSearching(s: boolean) {
    this.searching = s;
  }

  @action
  public setMeta(meta: IMeta) {
    this.meta = meta;
  }

  @action
  public setStatistics(statistics: IStatistics) {
    this.statistics = statistics;
  }

  @action
  public setFilter<T extends keyof IFilter>(key: T, value: IFilter[T]) {
    this.filter[key] = value;
  }
}

@injectable()
export class OmniService {
  public viewModel = new ViewModel();

  @inject()
  private canvasService: CanvasService;

  public init() {
    void this.getMeta();
    void this.getStatistics();
  }

  public async search() {
    this.viewModel.setSearching(true);
    const data = (
      await axios.get(API.search, {
        params: this.viewModel.filter,
      })
    ).data;
    this.canvasService.render({
      nodes: data.nodes,
      edges: [],
    });
    this.viewModel.setSearching(false);
  }

  private async getMeta() {
    const meta = (await axios.get<IMeta>(API.meta)).data;
    this.viewModel.setMeta(meta);
  }

  private async getStatistics() {
    const statistics = (await axios.get<IStatistics>(API.statistics)).data;
    this.viewModel.setStatistics(statistics);
  }
}
