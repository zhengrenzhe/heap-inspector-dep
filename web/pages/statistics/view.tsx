import React, { Component } from "react";
import { APIService } from "@web/service";
import { inject } from "@web/common";

export class StatisticsView extends Component {
  @inject()
  private apiService: APIService;

  public override async componentDidMount() {
    const data = await this.apiService.getStatistics();
    console.log(data);
  }

  public override render() {
    return <div></div>;
  }
}
