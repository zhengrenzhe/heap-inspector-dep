import React, { Component } from "react";
import { i18n } from "@/i18n";

export class Canvas extends Component {
  public render() {
    return (
      <div className="canvas-root">
        <div className="filter-panel">
          <div className="row">
            <label htmlFor="filter-constructor-name" className="filter-label">
              {i18n("filter-constructor-name")}
            </label>
            <input
              type="text"
              id="filter-constructor-name"
              className="filter-input"
              placeholder={i18n("eg: LogService")}
            />
          </div>

          <div className="row">
            <label htmlFor="filter-self-size" className="filter-label">
              {i18n("self-size")}
            </label>
            <select
              id="filter-self-size"
              className="filter-select"
              defaultValue="more-than"
            >
              <option value="more-than">{i18n("more-than")}</option>
              <option value="less-than">{i18n("less-than")}</option>
            </select>
            <input
              type="number"
              className="filter-input"
              placeholder={i18n("bytes")}
            />
          </div>

          <div className="row">
            <label htmlFor="filter-self-size" className="filter-label">
              {i18n("retain-size")}
            </label>
            <select
              id="filter-self-size"
              className="filter-select"
              defaultValue="more-than"
            >
              <option value="more-than">{i18n("more-than")}</option>
              <option value="less-than">{i18n("less-than")}</option>
            </select>
            <input
              type="number"
              className="filter-input"
              placeholder={i18n("bytes")}
            />
          </div>

          <div className="row">
            <label htmlFor="filter-self-size" className="filter-label">
              {i18n("reference-depth")}
            </label>
            <select
              id="filter-self-size"
              className="filter-select"
              defaultValue="more-than"
            >
              <option value="more-than">{i18n("more-than")}</option>
              <option value="less-than">{i18n("less-than")}</option>
            </select>
            <input
              type="number"
              className="filter-input"
              placeholder={i18n("eg-reference-depth")}
            />
          </div>
        </div>
        <div id="canvas" />
      </div>
    );
  }
}
