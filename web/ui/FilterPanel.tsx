import React, { ChangeEventHandler, Component } from "react";
import { observer } from "mobx-react";

import {
  CompareMode,
  IFilterCondition,
  SnapshotService,
  toNumber,
} from "@/service";
import { I18n, i18n } from "@/i18n";

import { Panel } from "./panel";

interface IRowProps {
  inputField: keyof IFilterCondition;
  labelName: keyof I18n;
  inputPlaceholder?: keyof I18n;
  inputIsNumber?: boolean;
  inputType?: "text" | "number" | "checkbox";
  selectField?: keyof IFilterCondition;
  compareMode?: boolean;
  inputIsCheckbox?: boolean;
}

@observer
class FilterRow extends Component<IRowProps> {
  private get value() {
    return SnapshotService.viewModel.filter[this.props.inputField] as any;
  }

  private onInputChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    SnapshotService.viewModel.setFilter(
      this.props.inputField,
      this.props.inputIsNumber
        ? toNumber(e.target.value)
        : this.props.inputIsCheckbox
        ? e.target.checked
        : e.target.value
    );
  };

  private onSelectChange: ChangeEventHandler<HTMLSelectElement> = (e) => {
    if (this.props.selectField) {
      const val = toNumber(e.target.value);
      SnapshotService.viewModel.setFilter(
        this.props.selectField,
        val === CompareMode.MoreThan
          ? CompareMode.MoreThan
          : CompareMode.LessThan
      );
    }
  };

  public render() {
    return (
      <div className="row">
        <label htmlFor={this.props.inputField} className="filter-label">
          {i18n(this.props.labelName)}
        </label>
        {this.props.compareMode && this.props.selectField ? (
          <select
            className="filter-select"
            defaultValue={
              SnapshotService.viewModel.filter[this.props.selectField] as any
            }
            onChange={this.onSelectChange}
          >
            {[
              { value: CompareMode.MoreThan, name: "more-than" as keyof I18n },
              { value: CompareMode.LessThan, name: "less-than" as keyof I18n },
            ].map((s, i) => (
              <option value={s.value} key={i}>
                {i18n(s.name)}
              </option>
            ))}
          </select>
        ) : null}
        <input
          type={this.props.inputType || "text"}
          id={this.props.inputField}
          min={0}
          className="filter-input"
          placeholder={
            this.props.inputPlaceholder ? i18n(this.props.inputPlaceholder) : ""
          }
          value={this.value}
          onChange={this.onInputChange}
          checked={this.value}
        />
      </div>
    );
  }
}

export class FilterPanel extends Component {
  public render() {
    return (
      <Panel name="filter">
        <FilterRow
          inputField="constructor_name"
          inputPlaceholder="eg: LogService"
          labelName="filter-constructor-name"
        />

        <FilterRow
          labelName="self-size"
          inputPlaceholder="bytes"
          inputType="number"
          inputField="self_size"
          selectField="self_size_compare_mode"
          compareMode
          inputIsNumber
        />

        <FilterRow
          labelName="retain-size"
          inputPlaceholder="bytes"
          inputType="number"
          inputField="retain_size"
          selectField="retain_size_compare_mode"
          compareMode
          inputIsNumber
        />

        <FilterRow
          labelName="reference-depth"
          inputPlaceholder="eg-reference-depth"
          inputType="number"
          inputField="reference_depth"
          selectField="reference_depth_compare_mode"
          compareMode
          inputIsNumber
        />

        <FilterRow
          labelName="nodes_limit"
          inputPlaceholder="nodes_limit"
          inputType="number"
          inputField="nodes_limit"
          inputIsNumber
        />

        <FilterRow
          labelName="ignore-system"
          inputPlaceholder="nodes_limit"
          inputType="checkbox"
          inputIsCheckbox
          inputField="ignore_system_node"
        />

        <button
          className="filter-apply-button"
          onClick={SnapshotService.refreshGraph}
        >
          {i18n("apply-filter")}
        </button>
      </Panel>
    );
  }
}
