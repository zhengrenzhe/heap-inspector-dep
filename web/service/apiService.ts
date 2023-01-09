import axios from "axios";

import { API, injectable } from "@web/common";

interface IStatistics {
  percent: Record<string, number>;
  total_bytes: number;
}

@injectable()
export class APIService {
  public async getStatistics() {
    return (await axios.get<IStatistics>(API.statistics)).data;
  }
}
