import axios from "axios";

import { API, injectable } from "@web/common";

export interface IStatistics {
  percent: Record<string, number>;
  total_bytes: number;
}

export interface IMeta {
  edge_count: number;
  node_count: number;
  file_size: number;
  file_path: string;
  node_types: string[];
  edge_types: string[];
}

@injectable()
export class APIService {
  public async getStatistics() {
    return (await axios.get<IStatistics>(API.statistics)).data;
  }

  public async getMeta() {
    return (await axios.get<IMeta>(API.meta)).data;
  }
}
