import React, { Component } from "react";
import { Panel, FilterRow } from "../atoms";
import { i18n } from "@/i18n";

interface IFilterPanelProps {
  onSubmit: () => void;
}

export class FilterPanel extends Component<IFilterPanelProps> {
  public render() {
    return (
      <Panel name="filter">
        <FilterRow field="constructor_name" placeholder="eg: LogService" />

        <FilterRow
          field="self_size"
          placeholder="bytes"
          inputType="number"
          selectField="self_size_compare_mode"
          compareMode
          inputIsNumber
        />

        <FilterRow
          field="retain_size"
          placeholder="bytes"
          inputType="number"
          selectField="retain_size_compare_mode"
          compareMode
          inputIsNumber
        />

        <FilterRow
          field="reference_depth"
          placeholder="eg-reference-depth"
          inputType="number"
          selectField="reference_depth_compare_mode"
          compareMode
          inputIsNumber
        />

        <FilterRow
          field="nodes_limit"
          placeholder="nodes_limit"
          inputType="number"
          inputIsNumber
        />

        <FilterRow
          field="ignore_system_node"
          placeholder="nodes_limit"
          inputType="checkbox"
          inputIsCheckbox
        />

        <button className="filter-apply-button" onClick={this.props.onSubmit}>
          {i18n("apply-filter")}
        </button>
      </Panel>
    );
  }
}
