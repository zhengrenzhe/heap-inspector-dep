import axios from "axios";
import { action, makeObservable, observable } from "mobx";

import { API, inject, injectable } from "@web/common";
import { CanvasService } from "@web/workbench/canvas/canvas.service";

interface IFilter {
  filter_from: string[];
  filter_name: string;
  self_size_mode: string;
  self_size: number;
  retained_size_mode: string;
  retained_size: number;
  depth: number;
}

interface IMeta {
  edge_count: number;
  node_count: number;
  file_size: number;
  file_name: string;
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
    depth: 0,
  };

  @observable
  public searching = false;

  constructor() {
    makeObservable(this);
  }

  @action
  public setSearching(s: boolean) {
    this.searching = s;
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

  public async search() {
    this.viewModel.setSearching(true);
    const data = (
      await axios.get(API.search, {
        params: this.viewModel.filter,
      })
    ).data;
    console.log(data);
    this.canvasService.render({
      nodes: data.nodes,
      edges: [],
    });
    this.viewModel.setSearching(false);
  }

  public async getMeta() {
    return (await axios.get<IMeta>(API.meta)).data;
  }
}