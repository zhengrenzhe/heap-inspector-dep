import React, { Component } from "react";
import { I18n, i18n } from "@/i18n";
import { SnapshotService } from "@/service";

interface IRowProps {
  id: string;
  labelName: keyof I18n;
  inputPlaceholder: keyof I18n;
  inputType?: "text" | "number";
  selectOptions?: { value: string; name: keyof I18n }[];
}
class FilterRow extends Component<IRowProps> {
  public render() {
    return (
      <div className="row">
        <label htmlFor={this.props.id} className="filter-label">
          {i18n(this.props.labelName)}
        </label>
        {this.props.selectOptions ? (
          <select>
            {this.props.selectOptions.map((s, i) => (
              <option value={s.value} key={i}>
                {i18n(s.name)}
              </option>
            ))}
          </select>
        ) : null}
        <input
          type={this.props.inputType || "text"}
          id={this.props.id}
          min={0}
          className="filter-input"
          placeholder={i18n(this.props.inputPlaceholder)}
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
        <div className="filter-panel">
          <FilterRow
            id="filter-constructor-name"
            inputPlaceholder="eg: LogService"
            labelName="filter-constructor-name"
          />

          <FilterRow
            id="filter-self-size"
            labelName="self-size"
            inputPlaceholder="bytes"
            inputType="number"
            selectOptions={[
              { value: "more-than", name: "more-than" },
              { value: "less-than", name: "less-than" },
            ]}
          />

          <FilterRow
            id="filter-retain-size"
            labelName="retain-size"
            inputPlaceholder="bytes"
            inputType="number"
            selectOptions={[
              { value: "more-than", name: "more-than" },
              { value: "less-than", name: "less-than" },
            ]}
          />

          <FilterRow
            id="filter-reference-depth"
            labelName="reference-depth"
            inputPlaceholder="eg-reference-depth"
            selectOptions={[
              { value: "more-than", name: "more-than" },
              { value: "less-than", name: "less-than" },
            ]}
            inputType="number"
          />
        </div>
        <div id="canvas" />
      </div>
    );
  }
}
