import React, { ChangeEventHandler, Component } from "react";
import { observer } from "mobx-react";

import { I18n, i18n } from "@/i18n";
import { CompareMode, IFilter, SnapshotService } from "@/service";

import { Panel } from "./panel";

interface IRowProps {
  inputField: keyof IFilter;
  labelName: keyof I18n;
  inputPlaceholder: keyof I18n;
  inputIsNumber?: boolean;
  inputType?: "text" | "number";
  selectField?: keyof IFilter;
  compareMode?: boolean;
}

@observer
class FilterRow extends Component<IRowProps> {
  private onInputChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    SnapshotService.viewModel.setFilter(
      this.props.inputField,
      this.props.inputIsNumber ? parseInt(e.target.value) : e.target.value
    );
  };

  private onSelectChange: ChangeEventHandler<HTMLSelectElement> = (e) => {
    if (this.props.selectField) {
      const val = parseInt(e.target.value);
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
              SnapshotService.viewModel.filter[this.props.selectField]
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
          placeholder={i18n(this.props.inputPlaceholder)}
          value={SnapshotService.viewModel.filter[this.props.inputField]}
          onChange={this.onInputChange}
        />
      </div>
    );
  }
}

export class Canvas extends Component {
  public componentDidMount() {
    SnapshotService.init();
  }

  public render() {
    return (
      <div className="canvas-root">
        <div className="panels">
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

            <button className="filter-apply-button">
              {i18n("apply-filter")}
            </button>
          </Panel>
        </div>
        <div id="canvas" />
      </div>
    );
  }
}
